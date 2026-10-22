#include <string.h>
#include <stdio.h>
#include "main.h"
#include "shared.h"
#include <time.h>

char temp_buf[TEMP_BUFSIZE];

uint32_t millis(void)
{
  return htim2.Instance->CNT;
}

void delay_ms(uint32_t amount)
{
  if(amount == 0)
    return;
  uint32_t start = millis();
  while ((millis() - start) < amount)
  {
    ;
  }
}

char* goto_next_arg(char* buf, char* buf_end)
{
  char* curr = buf;  
  if(buf == NULL || curr >= buf_end)
    return NULL;
  while(curr < buf_end && *curr != ' ')
      curr++;
  while(curr < buf_end && *curr == ' ')
      curr++;
  if(curr >= buf_end)
    return NULL;
  return curr;
}

void strip_newline(char* line, uint32_t size)
{
  for(uint32_t i = 0; i < size; ++i)
    if(line[i] == '\n' || line[i] == '\r')
      line[i] = 0;
}

void idle_loop(void)
{
  while(1)
    delay_ms(1000);
}

uint32_t get_uuid(void)
{
  return (*STM32F0_UUID0) ^ (*STM32F0_UUID1) ^ (*STM32F0_UUID2);
}

uint8_t is_rtc_valid(void)
{
  return HAL_RTCEx_BKUPRead(&hrtc, RTC_MAGICNUM_REG) == RTC_MAGIC_NUMBER;
}

void mark_rtc_as_valid(void)
{
  HAL_PWR_EnableBkUpAccess();
  HAL_RTCEx_BKUPWrite(&hrtc, RTC_MAGICNUM_REG, RTC_MAGIC_NUMBER);
  HAL_PWR_DisableBkUpAccess();
}

void set_utc_offset(int16_t minutes)
{
  HAL_PWR_EnableBkUpAccess();
  HAL_RTCEx_BKUPWrite(&hrtc, RTC_UTC_OFFSET_REG, (uint32_t)minutes);
  HAL_PWR_DisableBkUpAccess();
}

int16_t get_utc_offset(void)
{
  return (int16_t)HAL_RTCEx_BKUPRead(&hrtc, RTC_UTC_OFFSET_REG);
}

static struct tm shared_tm_buf;

static void epoch_to_tm(uint32_t ts, struct tm *out)
{
  uint32_t secs = ts % 86400U;
  uint32_t days = ts / 86400U;
  out->tm_sec = (int)(secs % 60U);
  out->tm_min = (int)((secs / 60U) % 60U);
  out->tm_hour = (int)(secs / 3600U);
  out->tm_wday = (int)((days + 4U) % 7U);

  int32_t z = (int32_t)days + 719468;
  int32_t era = (z >= 0 ? z : z - 146096) / 146097;
  uint32_t doe = (uint32_t)(z - era * 146097);
  uint32_t yoe = (doe - doe / 1460U + doe / 36524U - doe / 146096U) / 365U;
  int32_t y = (int32_t)yoe + era * 400;
  uint32_t doy = doe - (365U * yoe + yoe / 4U - yoe / 100U);
  uint32_t mp = (5U * doy + 2U) / 153U;
  uint32_t d = doy - (153U * mp + 2U) / 5U + 1U;
  uint32_t m = mp < 10U ? mp + 3U : mp - 9U;
  y += (m <= 2U) ? 1 : 0;

  out->tm_year = (int)(y - 1900);
  out->tm_mon = (int)(m - 1U);
  out->tm_mday = (int)d;
  out->tm_isdst = 0;
}

uint8_t RTC_SetFromUnixTimestamp(RTC_HandleTypeDef *rtc_ptr, uint32_t unix_timestamp)
{
  RTC_TimeTypeDef sTime = {0};
  RTC_DateTypeDef sDate = {0};
  struct tm *time_info = &shared_tm_buf;
  epoch_to_tm(unix_timestamp, time_info);
  HAL_PWR_EnableBkUpAccess();
  sTime.Hours = time_info->tm_hour;
  sTime.Minutes = time_info->tm_min;
  sTime.Seconds = time_info->tm_sec;
  sTime.DayLightSaving = RTC_DAYLIGHTSAVING_NONE;
  sTime.StoreOperation = RTC_STOREOPERATION_RESET;
  HAL_RTC_SetTime(rtc_ptr, &sTime, RTC_FORMAT_BIN);
  if (time_info->tm_wday == 0)
    sDate.WeekDay = RTC_WEEKDAY_SUNDAY;
  else
    sDate.WeekDay = (uint8_t)time_info->tm_wday;
  sDate.Month = (uint8_t)(time_info->tm_mon + 1);
  sDate.Date = (uint8_t)time_info->tm_mday;
  sDate.Year = (uint8_t)(time_info->tm_year - 100);
  HAL_RTC_SetDate(rtc_ptr, &sDate, RTC_FORMAT_BIN);
  HAL_PWR_DisableBkUpAccess();
  return 0;
}

uint32_t get_unix_ts(RTC_HandleTypeDef *rtc_ptr)
{
  RTC_TimeTypeDef sTime = {0};
  RTC_DateTypeDef sDate = {0};

  HAL_RTC_GetTime(rtc_ptr, &sTime, RTC_FORMAT_BIN);
  HAL_RTC_GetDate(rtc_ptr, &sDate, RTC_FORMAT_BIN);

  int32_t y = (int32_t)sDate.Year + 2000;
  uint32_t m = (uint32_t)sDate.Month;
  uint32_t d = (uint32_t)sDate.Date;
  y -= (m <= 2U) ? 1 : 0;
  int32_t era = (y >= 0 ? y : y - 399) / 400;
  uint32_t yoe = (uint32_t)(y - era * 400);
  uint32_t doy = (153U * (m > 2U ? m - 3U : m + 9U) + 2U) / 5U + d - 1U;
  uint32_t doe = yoe * 365U + yoe / 4U - yoe / 100U + doy;
  int32_t days = era * 146097 + (int32_t)doe - 719468;
  if (days < 0)
    days = 0;

  return (uint32_t)days * 86400U
    + (uint32_t)sTime.Hours * 3600U
    + (uint32_t)sTime.Minutes * 60U
    + (uint32_t)sTime.Seconds;
}

struct tm* get_local_time(RTC_HandleTypeDef *rtc_ptr, int16_t offset_minutes)
{
  uint32_t utc_epoch = get_unix_ts(rtc_ptr);
  int64_t local_epoch = (int64_t)utc_epoch + ((int64_t)offset_minutes * 60);
  if (local_epoch < 0)
    local_epoch = 0;
  epoch_to_tm((uint32_t)local_epoch, &shared_tm_buf);
  return &shared_tm_buf;
}
