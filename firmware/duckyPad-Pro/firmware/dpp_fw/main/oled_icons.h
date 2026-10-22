#ifndef __OLED_ICONS_H__
#define __OLED_ICONS_H__

#include <stdint.h>
#include "ssd1306.h"

typedef struct {
    uint32_t codepoint;
    uint8_t bmp16[32];    // 16x16 1-bit monochrome bitmap (2 bytes/row)
    uint8_t bmp10[20];    // 10x10 1-bit monochrome bitmap (2 bytes/row)
    uint16_t glyph7x10[10]; // 7x10 inline font glyph (MSB-aligned)
    uint16_t glyph6x8[8];   // 6x8 inline font glyph (MSB-aligned)
} oled_icon_entry_t;

const oled_icon_entry_t* oled_find_icon(uint32_t codepoint);
uint32_t oled_utf8_next_codepoint(const char** str_ptr);
uint8_t oled_utf8_glyph_count(const char* str);
void oled_utf8_truncate_glyphs(char* str, uint8_t max_glyphs);
uint8_t oled_is_single_icon_str(const char* str, const oled_icon_entry_t** out_icon);
void oled_draw_icon_glyph(const oled_icon_entry_t* icon, FontDef font, SSD1306_COLOR color);

#endif // __OLED_ICONS_H__
