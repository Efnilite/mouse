//
// Created by efy on 13/09/25.
//

#ifndef PARSER_H
#define PARSER_H

#include <stdbool.h>
#include <stdint.h>

/**
 * Populates a walls array.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @param walls A pointer to a walls array.
 */
void parser_get_walls(uint8_t x, uint8_t y, bool* walls);

#endif //PARSER_H
