//
// Created by efy on 13/09/25.
//

#ifndef PATH_H
#define PATH_H

#include "maze.h"
#include "vec2i.h"

Point* grid_calculate_path(const Grid* grid, const Point* history, uint8_t history_size, const Vec2i* pos);

#endif //PATH_H
