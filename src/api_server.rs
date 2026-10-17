use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use cxx_qt::CxxQtThread;

use crate::bridge::device_controller::qobject;

#[derive(Debug, Clone)]
pub struct RestApiServerConfig {
    pub enabled: bool,
    pub bind_ip: String,
    pub port: u16,
    pub require_auth: bool,
    pub token: String,
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
        let _ = TcpStream::connect_timeout(
            &handle
                .bind_addr
                .parse()
                .unwrap_or_else(|_| std::net::SocketAddr::from(([127, 0, 0, 1], 28491))),
            Duration::from_millis(50),
        );
        if let Some(jh) = handle.join_handle.take() {
            let _ = jh.join();
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
    let _ = listener.set_nonblocking(true);

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
    while !stop_flag.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _peer)) => {
                if stop_flag.load(Ordering::Relaxed) {
                    break;
                }
                let req_cfg = cfg.clone();
                let req_qt = qt_thread.clone();
                std::thread::spawn(move || {
                    handle_client_connection(stream, &req_cfg, req_qt);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(35));
            }
            Err(_) => {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

struct ParsedHttpRequest {
    method: String,
    path: String,
    query: String,
    headers: HashMap<String, String>,
    body: String,
}

fn read_http_request(stream: &mut TcpStream) -> Result<ParsedHttpRequest, String> {
    let _ = stream.set_nonblocking(false);
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));

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
                if buf.len() > 64 * 1024 {
                    return Err("Request headers too large".to_string());
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(format!("Read error: {e}")),
        }
    }

    let Some(hdr_end) = header_end_pos else {
        return Err("Incomplete HTTP request headers".to_string());
    };

    let header_str = String::from_utf8_lossy(&buf[..hdr_end]).to_string();
    let mut lines = header_str.lines();
    let request_line = lines.next().ok_or_else(|| "Empty request line".to_string())?;
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

    let content_len = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0)
        .min(2 * 1024 * 1024);

    while buf.len() < hdr_end + content_len && start.elapsed() < Duration::from_secs(5) {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => buf.extend_from_slice(&temp[..n]),
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
        method,
        path,
        query,
        headers,
        body,
    })
}

fn write_http_response(stream: &mut TcpStream, status_code: u16, content_type: &str, body: &str) {
    let reason = match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
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
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(bytes);
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
        Err(e) => {
            let err_json = serde_json::json!({
                "ok": false,
                "error": e
            })
            .to_string();
            write_http_response(&mut stream, 400, "application/json; charset=utf-8", &err_json);
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
        let _ = tx.send(res);
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

    match rx.recv_timeout(Duration::from_secs(25)) {
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
