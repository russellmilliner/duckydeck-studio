use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use cxx_qt::CxxQtThread;
use serde::{Deserialize, Serialize};

use crate::bridge::device_controller::qobject;

pub const MAX_CONCURRENT_CONNECTIONS: usize = 8;
pub const MAX_HEADER_BYTES: usize = 64 * 1024;
pub const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;
pub const CONTROLLER_WAIT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_STORED_JOBS: usize = 256;

#[derive(Debug, Clone)]
pub struct RestApiServerConfig {
    pub enabled: bool,
    pub bind_ip: String,
    pub port: u16,
    pub require_auth: bool,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiJobRecord {
    pub id: String,
    pub operation: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub ok: Option<bool>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

struct JobStore {
    order: VecDeque<String>,
    jobs: HashMap<String, ApiJobRecord>,
}

impl Default for JobStore {
    fn default() -> Self {
        Self {
            order: VecDeque::with_capacity(MAX_STORED_JOBS),
            jobs: HashMap::with_capacity(MAX_STORED_JOBS),
        }
    }
}

static JOB_STORE: OnceLock<Mutex<JobStore>> = OnceLock::new();
static NEXT_JOB_ID: AtomicU64 = AtomicU64::new(1);

fn job_store() -> &'static Mutex<JobStore> {
    JOB_STORE.get_or_init(|| Mutex::new(JobStore::default()))
}

fn now_iso_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string()
}

pub fn create_api_job(operation: &str) -> ApiJobRecord {
    let seq = NEXT_JOB_ID.fetch_add(1, Ordering::Relaxed);
    let id = format!("job-{seq}");
    let ts = now_iso_timestamp();
    let record = ApiJobRecord {
        id: id.clone(),
        operation: operation.to_string(),
        status: "queued".to_string(),
        created_at: ts.clone(),
        updated_at: ts,
        ok: None,
        message: format!("Job '{operation}' accepted and queued"),
        result: None,
    };

    if let Ok(mut store) = job_store().lock() {
        while store.order.len() >= MAX_STORED_JOBS {
            if let Some(oldest) = store.order.pop_front() {
                store.jobs.remove(&oldest);
            }
        }
        store.order.push_back(id);
        store.jobs.insert(record.id.clone(), record.clone());
    }
    record
}

pub fn update_api_job(
    job_id: &str,
    status: &str,
    ok: Option<bool>,
    message: impl Into<String>,
    result: Option<serde_json::Value>,
) {
    if let Ok(mut store) = job_store().lock() {
        if let Some(job) = store.jobs.get_mut(job_id) {
            job.status = status.to_string();
            job.updated_at = now_iso_timestamp();
            job.ok = ok;
            job.message = message.into();
            if result.is_some() {
                job.result = result;
            }
        }
    }
}

pub fn get_api_job(job_id: &str) -> Option<ApiJobRecord> {
    job_store()
        .lock()
        .ok()
        .and_then(|store| store.jobs.get(job_id).cloned())
}

pub fn format_job_accepted_response(job: &ApiJobRecord) -> (u16, String) {
    (
        202,
        serde_json::json!({
            "ok": true,
            "accepted": true,
            "jobId": job.id,
            "operation": job.operation,
            "status": job.status,
            "statusUrl": format!("/api/v1/jobs/{}", job.id),
            "message": job.message,
        })
        .to_string(),
    )
}

pub fn handle_get_job_status(job_id: &str) -> (u16, String) {
    let clean_id = job_id.trim();
    if let Some(job) = get_api_job(clean_id) {
        (
            200,
            serde_json::json!({
                "ok": true,
                "job": job,
            })
            .to_string(),
        )
    } else {
        (
            404,
            serde_json::json!({
                "ok": false,
                "error": format!("Job '{clean_id}' not found"),
            })
            .to_string(),
        )
    }
}

pub fn try_acquire_connection_slot(active: &AtomicUsize) -> bool {
    try_acquire_connection_slot_with_max(active, MAX_CONCURRENT_CONNECTIONS)
}

pub fn try_acquire_connection_slot_with_max(active: &AtomicUsize, max: usize) -> bool {
    let mut current = active.load(Ordering::Acquire);
    loop {
        if current >= max {
            return false;
        }
        match active.compare_exchange_weak(
            current,
            current + 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return true,
            Err(actual) => current = actual,
        }
    }
}

pub struct ActiveConnectionGuard {
    counter: Arc<AtomicUsize>,
}

impl ActiveConnectionGuard {
    pub fn try_new(counter: Arc<AtomicUsize>, max: usize) -> Option<Self> {
        if try_acquire_connection_slot_with_max(&counter, max) {
            Some(Self { counter })
        } else {
            None
        }
    }
}

impl Drop for ActiveConnectionGuard {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::AcqRel);
    }
}

struct RunningServerHandle {
    stop_flag: Arc<AtomicBool>,
    bind_addr: String,
    join_handle: Option<std::thread::JoinHandle<()>>,
}

static SERVER_HANDLE: OnceLock<Mutex<Option<RunningServerHandle>>> = OnceLock::new();
static SERVER_STATUS: OnceLock<Mutex<String>> = OnceLock::new();

fn server_mutex() -> &'static Mutex<Option<RunningServerHandle>> {
    SERVER_HANDLE.get_or_init(|| Mutex::new(None))
}

fn status_mutex() -> &'static Mutex<String> {
    SERVER_STATUS.get_or_init(|| Mutex::new("Disabled".to_string()))
}

pub fn current_rest_api_status() -> String {
    status_mutex()
        .lock()
        .map(|s| s.clone())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn set_rest_api_status(status: impl Into<String>) {
    if let Ok(mut guard) = status_mutex().lock() {
        *guard = status.into();
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() || a.is_empty() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub fn stop_rest_api_server() {
    let prev = server_mutex().lock().ok().and_then(|mut g| g.take());
    if let Some(mut handle) = prev {
        handle.stop_flag.store(true, Ordering::SeqCst);
        let addr = handle
            .bind_addr
            .parse()
            .unwrap_or_else(|_| std::net::SocketAddr::from(([127, 0, 0, 1], 28491)));
        let _ = TcpStream::connect_timeout(&addr, Duration::from_millis(50)); // ignore: wakeup dummy connect to unblock accept() on shutdown
        if let Some(jh) = handle.join_handle.take() {
            let _ = jh.join(); // ignore: best-effort accept thread join during shutdown
        }
    }
    set_rest_api_status("Disabled");
}

pub fn start_or_restart_rest_api_server(
    cfg: RestApiServerConfig,
    qt_thread: CxxQtThread<qobject::DeviceController>,
) -> Result<String, String> {
    stop_rest_api_server();

    if !cfg.enabled {
        let msg = "HTTP REST API Server is disabled".to_string();
        set_rest_api_status("Disabled");
        return Ok(msg);
    }

    let clean_ip = if cfg.bind_ip.trim().is_empty() {
        "127.0.0.1".to_string()
    } else {
        cfg.bind_ip.trim().to_string()
    };
    let port = if cfg.port == 0 { 28491 } else { cfg.port };
    let bind_addr = format!("{clean_ip}:{port}");

    let listener = TcpListener::bind(&bind_addr).map_err(|e| {
        let err = format!("Failed binding HTTP REST API to {bind_addr}: {e}");
        set_rest_api_status(format!("Error: {err}"));
        err
    })?;
    if let Err(e) = listener.set_nonblocking(false) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed configuring blocking mode on REST API listener: {e}"),
        );
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let thread_stop = Arc::clone(&stop_flag);
    let thread_addr = bind_addr.clone();
    let thread_cfg = cfg.clone();

    let join_handle = std::thread::Builder::new()
        .name("duckydeck-rest-api".to_string())
        .spawn(move || {
            run_accept_loop(listener, thread_stop, thread_cfg, qt_thread);
        })
        .map_err(|e| {
            let err = format!("Failed spawning HTTP REST API thread: {e}");
            set_rest_api_status(format!("Error: {err}"));
            err
        })?;

    if let Ok(mut guard) = server_mutex().lock() {
        *guard = Some(RunningServerHandle {
            stop_flag,
            bind_addr: thread_addr,
            join_handle: Some(join_handle),
        });
    }

    let auth_mode = if cfg.require_auth {
        "Bearer Token Auth Required"
    } else {
        "Unauthenticated (WARNING)"
    };
    let status = format!("Listening on http://{bind_addr} ({auth_mode})");
    set_rest_api_status(&status);
    Ok(status)
}

fn run_accept_loop(
    listener: TcpListener,
    stop_flag: Arc<AtomicBool>,
    cfg: RestApiServerConfig,
    qt_thread: CxxQtThread<qobject::DeviceController>,
) {
    let active_connections = Arc::new(AtomicUsize::new(0));

    while !stop_flag.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((mut stream, _peer)) => {
                if stop_flag.load(Ordering::Acquire) {
                    break;
                }
                let Some(conn_guard) = ActiveConnectionGuard::try_new(
                    Arc::clone(&active_connections),
                    MAX_CONCURRENT_CONNECTIONS,
                ) else {
                    let err_json = serde_json::json!({
                        "ok": false,
                        "error": format!(
                            "Service Unavailable: maximum concurrent connections ({MAX_CONCURRENT_CONNECTIONS}) reached"
                        )
                    })
                    .to_string();
                    write_http_response(
                        &mut stream,
                        503,
                        "application/json; charset=utf-8",
                        &err_json,
                    );
                    let _ = stream.shutdown(Shutdown::Both); // ignore: best-effort socket close after 503 rejection
                    continue;
                };

                let req_cfg = cfg.clone();
                let req_qt = qt_thread.clone();
                std::thread::spawn(move || {
                    let _guard = conn_guard;
                    handle_client_connection(stream, &req_cfg, req_qt);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => {
                if stop_flag.load(Ordering::Acquire) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpReadError {
    BadRequest(String),
    PayloadTooLarge(String),
}

impl HttpReadError {
    pub fn status_code(&self) -> u16 {
        match self {
            HttpReadError::BadRequest(_) => 400,
            HttpReadError::PayloadTooLarge(_) => 413,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            HttpReadError::BadRequest(msg) | HttpReadError::PayloadTooLarge(msg) => msg,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedHttpHeaders {
    pub method: String,
    pub path: String,
    pub query: String,
    pub headers: HashMap<String, String>,
    pub content_length: usize,
}

pub fn parse_and_validate_http_headers(
    header_bytes: &[u8],
    max_header_bytes: usize,
    max_body_bytes: usize,
) -> Result<ParsedHttpHeaders, HttpReadError> {
    if header_bytes.len() > max_header_bytes {
        return Err(HttpReadError::BadRequest(
            "Request headers too large".to_string(),
        ));
    }

    let header_str = String::from_utf8_lossy(header_bytes).to_string();
    let mut lines = header_str.lines();
    let request_line = lines
        .next()
        .ok_or_else(|| HttpReadError::BadRequest("Empty request line".to_string()))?;
    if request_line.trim().is_empty() {
        return Err(HttpReadError::BadRequest("Empty request line".to_string()));
    }

    let mut req_parts = request_line.split_whitespace();
    let method = req_parts
        .next()
        .unwrap_or("GET")
        .trim()
        .to_ascii_uppercase();
    let raw_target = req_parts.next().unwrap_or("/").trim();
    let (path, query) = match raw_target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (raw_target.to_string(), String::new()),
    };

    let mut headers = HashMap::new();
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Some((k, v)) = t.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    let content_length = if let Some(raw_cl) = headers.get("content-length") {
        let parsed = raw_cl.trim().parse::<u64>().map_err(|_| {
            HttpReadError::BadRequest(format!("Invalid Content-Length header: '{raw_cl}'"))
        })?;
        if parsed > max_body_bytes as u64 {
            return Err(HttpReadError::PayloadTooLarge(format!(
                "Payload Too Large: Content-Length ({parsed} bytes) exceeds maximum allowed body size of {max_body_bytes} bytes"
            )));
        }
        parsed as usize
    } else {
        0
    };

    Ok(ParsedHttpHeaders {
        method,
        path,
        query,
        headers,
        content_length,
    })
}

struct ParsedHttpRequest {
    method: String,
    path: String,
    query: String,
    headers: HashMap<String, String>,
    body: String,
}

fn read_http_request(stream: &mut TcpStream) -> Result<ParsedHttpRequest, HttpReadError> {
    if let Err(e) = stream.set_nonblocking(false) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed setting HTTP client socket blocking mode: {e}"),
        );
    }
    if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(5))) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed setting HTTP client read timeout: {e}"),
        );
    }
    if let Err(e) = stream.set_write_timeout(Some(Duration::from_secs(10))) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed setting HTTP client write timeout: {e}"),
        );
    }

    let mut buf = Vec::with_capacity(8192);
    let mut temp = [0u8; 4096];
    let mut header_end_pos: Option<usize> = None;
    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(5) {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&temp[..n]);
                if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                    header_end_pos = Some(pos + 4);
                    break;
                }
                if buf.len() > MAX_HEADER_BYTES {
                    return Err(HttpReadError::BadRequest(
                        "Request headers too large".to_string(),
                    ));
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(HttpReadError::BadRequest(format!("Read error: {e}"))),
        }
    }

    let Some(hdr_end) = header_end_pos else {
        return Err(HttpReadError::BadRequest(
            "Incomplete HTTP request headers".to_string(),
        ));
    };

    let parsed_headers =
        parse_and_validate_http_headers(&buf[..hdr_end], MAX_HEADER_BYTES, MAX_BODY_BYTES)?;
    let content_len = parsed_headers.content_length;

    if buf.len().saturating_sub(hdr_end) > MAX_BODY_BYTES {
        return Err(HttpReadError::PayloadTooLarge(format!(
            "Payload Too Large: request body exceeds maximum allowed size of {MAX_BODY_BYTES} bytes"
        )));
    }

    while buf.len() < hdr_end + content_len && start.elapsed() < Duration::from_secs(5) {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&temp[..n]);
                if buf.len().saturating_sub(hdr_end) > MAX_BODY_BYTES {
                    return Err(HttpReadError::PayloadTooLarge(format!(
                        "Payload Too Large: request body exceeds maximum allowed size of {MAX_BODY_BYTES} bytes"
                    )));
                }
            }
            Err(_) => break,
        }
    }

    let body_slice = if buf.len() >= hdr_end {
        let end = (hdr_end + content_len).min(buf.len());
        &buf[hdr_end..end]
    } else {
        &[]
    };
    let body = String::from_utf8_lossy(body_slice).to_string();

    Ok(ParsedHttpRequest {
        method: parsed_headers.method,
        path: parsed_headers.path,
        query: parsed_headers.query,
        headers: parsed_headers.headers,
        body,
    })
}

fn write_http_response(stream: &mut TcpStream, status_code: u16, content_type: &str, body: &str) {
    let reason = match status_code {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "OK",
    };
    let bytes = body.as_bytes();
    let header = format!(
        "HTTP/1.1 {status_code} {reason}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, PATCH, DELETE, OPTIONS\r\n\
         Access-Control-Allow-Headers: Authorization, Content-Type, X-API-Token\r\n\
         Connection: close\r\n\r\n",
        bytes.len()
    );
    if let Err(e) = stream.write_all(header.as_bytes()) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed writing HTTP response headers: {e}"),
        );
        return;
    }
    if let Err(e) = stream.write_all(bytes) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "REST_API",
            format!("Failed writing HTTP response body: {e}"),
        );
    }
    let _ = stream.flush();
}

fn extract_bearer_token(req: &ParsedHttpRequest) -> Option<String> {
    if let Some(auth) = req.headers.get("authorization") {
        let trimmed = auth.trim();
        if let Some(rest) = trimmed
            .strip_prefix("Bearer ")
            .or_else(|| trimmed.strip_prefix("bearer "))
        {
            return Some(rest.trim().to_string());
        }
    }
    if let Some(tok) = req.headers.get("x-api-token") {
        return Some(tok.trim().to_string());
    }
    None
}

fn handle_client_connection(
    mut stream: TcpStream,
    cfg: &RestApiServerConfig,
    qt_thread: CxxQtThread<qobject::DeviceController>,
) {
    let req = match read_http_request(&mut stream) {
        Ok(r) => r,
        Err(err) => {
            let status = err.status_code();
            let err_json = serde_json::json!({
                "ok": false,
                "error": err.message()
            })
            .to_string();
            write_http_response(
                &mut stream,
                status,
                "application/json; charset=utf-8",
                &err_json,
            );
            return;
        }
    };

    if req.method == "OPTIONS" {
        write_http_response(&mut stream, 204, "text/plain", "");
        return;
    }

    // Unauthenticated health probe
    if req.method == "GET" && req.path == "/api/v1/health" {
        let body = serde_json::json!({
            "ok": true,
            "service": "duckydeck-studio",
            "version": crate::bridge::device_controller::APP_VERSION,
            "authRequired": cfg.require_auth
        })
        .to_string();
        write_http_response(&mut stream, 200, "application/json; charset=utf-8", &body);
        return;
    }

    // Unauthenticated OpenAPI 3.0 / Swagger specifications
    if req.method == "GET"
        && (req.path == "/api/v1/openapi.json" || req.path == "/api/v1/swagger.json")
    {
        const SWAGGER_JSON: &str = include_str!("../docs/swagger.json");
        write_http_response(
            &mut stream,
            200,
            "application/json; charset=utf-8",
            SWAGGER_JSON,
        );
        return;
    }

    if req.method == "GET" && req.path == "/api/v1/openapi.yaml" {
        const OPENAPI_YAML: &str = include_str!("../docs/openapi.yaml");
        write_http_response(
            &mut stream,
            200,
            "application/yaml; charset=utf-8",
            OPENAPI_YAML,
        );
        return;
    }

    // Enforce Bearer token authentication when enabled
    if cfg.require_auth {
        let expected = cfg.token.trim().as_bytes();
        let provided = extract_bearer_token(&req).unwrap_or_default();
        if !constant_time_eq(provided.as_bytes(), expected) {
            let err_json = serde_json::json!({
                "ok": false,
                "error": "Unauthorized: valid API token required via 'Authorization: Bearer <token>' or 'X-API-Token: <token>' header."
            })
            .to_string();
            write_http_response(&mut stream, 401, "application/json; charset=utf-8", &err_json);
            return;
        }
    }

    let clean_path = req.path.trim().trim_end_matches('/');
    if req.method == "GET" {
        if let Some(job_id) = clean_path.strip_prefix("/api/v1/jobs/") {
            if !job_id.is_empty() && !job_id.contains('/') {
                let (status_code, json_body) = handle_get_job_status(job_id);
                write_http_response(
                    &mut stream,
                    status_code,
                    "application/json; charset=utf-8",
                    &json_body,
                );
                return;
            }
        }
    }

    // Dispatch onto the Qt main thread where DeviceController lives
    let (tx, rx) = mpsc::sync_channel::<(u16, String)>(1);
    let method = req.method;
    let path = req.path;
    let query = req.query;
    let body = req.body;

    let queue_res = qt_thread.queue(move |mut ctrl| {
        let res = ctrl
            .as_mut()
            .handle_rest_api_request(&method, &path, &query, &body);
        if let Err(e) = tx.send(res) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "REST_API",
                format!("HTTP client disconnected before controller response: {e}"),
            );
        }
    });

    if queue_res.is_err() {
        let err_json = serde_json::json!({
            "ok": false,
            "error": "Internal error: failed queueing request to controller thread"
        })
        .to_string();
        write_http_response(&mut stream, 500, "application/json; charset=utf-8", &err_json);
        return;
    }

    match rx.recv_timeout(CONTROLLER_WAIT_TIMEOUT) {
        Ok((status_code, json_body)) => {
            write_http_response(
                &mut stream,
                status_code,
                "application/json; charset=utf-8",
                &json_body,
            );
        }
        Err(_) => {
            let err_json = serde_json::json!({
                "ok": false,
                "error": "Request timed out waiting for controller response"
            })
            .to_string();
            write_http_response(&mut stream, 504, "application/json; charset=utf-8", &err_json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_length_exceeding_2mb_returns_413() {
        let over_limit = MAX_BODY_BYTES + 1;
        let raw_headers = format!(
            "POST /api/v1/profiles HTTP/1.1\r\nHost: 127.0.0.1:28491\r\nContent-Type: application/json\r\nContent-Length: {over_limit}\r\n\r\n"
        );
        let err = parse_and_validate_http_headers(
            raw_headers.as_bytes(),
            MAX_HEADER_BYTES,
            MAX_BODY_BYTES,
        )
        .expect_err("Content-Length > 2 MB must be rejected with 413");
        assert_eq!(err.status_code(), 413);
        assert!(matches!(err, HttpReadError::PayloadTooLarge(_)));

        let exact_limit = MAX_BODY_BYTES;
        let ok_headers = format!(
            "POST /api/v1/profiles HTTP/1.1\r\nHost: 127.0.0.1:28491\r\nContent-Length: {exact_limit}\r\n\r\n"
        );
        let parsed = parse_and_validate_http_headers(
            ok_headers.as_bytes(),
            MAX_HEADER_BYTES,
            MAX_BODY_BYTES,
        )
        .expect("Content-Length == 2 MB must be allowed");
        assert_eq!(parsed.content_length, MAX_BODY_BYTES);
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.path, "/api/v1/profiles");
    }

    #[test]
    fn test_connection_cap_limits_concurrent_slots_to_8() {
        let active = Arc::new(AtomicUsize::new(0));
        let mut guards = Vec::new();

        for i in 0..MAX_CONCURRENT_CONNECTIONS {
            let guard = ActiveConnectionGuard::try_new(
                Arc::clone(&active),
                MAX_CONCURRENT_CONNECTIONS,
            );
            assert!(
                guard.is_some(),
                "Expected slot #{i} (< {MAX_CONCURRENT_CONNECTIONS}) to be acquired"
            );
            guards.push(guard.unwrap());
        }
        assert_eq!(active.load(Ordering::SeqCst), MAX_CONCURRENT_CONNECTIONS);

        // 9th connection must be rejected (triggers 503)
        assert!(
            !try_acquire_connection_slot(&active),
            "9th concurrent connection must be rejected when 8 are active"
        );
        assert!(
            ActiveConnectionGuard::try_new(Arc::clone(&active), MAX_CONCURRENT_CONNECTIONS)
                .is_none()
        );

        // Dropping one guard releases a slot and allows one new connection
        guards.pop();
        assert_eq!(
            active.load(Ordering::SeqCst),
            MAX_CONCURRENT_CONNECTIONS - 1
        );
        let next_guard =
            ActiveConnectionGuard::try_new(Arc::clone(&active), MAX_CONCURRENT_CONNECTIONS);
        assert!(
            next_guard.is_some(),
            "After releasing a slot, a new connection must be accepted"
        );
    }

    #[test]
    fn test_async_job_lifecycle_and_status_lookup() {
        let job = create_api_job("sd/sync");
        assert_eq!(job.status, "queued");
        let (code, body) = format_job_accepted_response(&job);
        assert_eq!(code, 202);
        assert!(body.contains(&job.id));

        update_api_job(
            &job.id,
            "completed",
            Some(true),
            "Sync finished",
            Some(serde_json::json!({ "filesWritten": 4 })),
        );

        let (get_code, get_body) = handle_get_job_status(&job.id);
        assert_eq!(get_code, 200);
        let val: serde_json::Value = serde_json::from_str(&get_body).unwrap();
        assert_eq!(val["job"]["status"], "completed");
        assert_eq!(val["job"]["ok"], true);
    }
}
