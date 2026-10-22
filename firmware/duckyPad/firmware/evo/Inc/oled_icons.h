#ifndef __OLED_ICONS_H__
#define __OLED_ICONS_H__

#include <stdint.h>
#include "ssd1306.h"

typedef struct {
    uint32_t codepoint;
    uint8_t glyph6x8[8];
} oled_icon_entry_t;

const oled_icon_entry_t* oled_find_icon(uint32_t codepoint);
uint32_t oled_utf8_next_codepoint(const char** str_ptr);
uint8_t oled_utf8_glyph_count(const char* str);
void oled_utf8_truncate_glyphs(char* str, uint8_t max_glyphs);
void oled_draw_icon_glyph(const oled_icon_entry_t* icon, FontDef font, SSD1306_COLOR color);

#endif // __OLED_ICONS_H__
