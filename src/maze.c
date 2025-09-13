//
// Created by efy on 12/09/25.
//

#include "vec2i.h"
#include "maze.h"

#include <math.h>
#include <stdlib.h>
#include <string.h>

Point* grid_geti(const Grid* grid, const uint8_t x, const uint8_t y) {
    const uint8_t i = x + y * HEIGHT;

    if (i < 0 || i > SIZE) {
        return NULL;
    }

    return &grid->points[i];
}

static uint8_t ceili(const double cx) {
    return (int)ceil(cx);
}

Point* grid_getd(const Grid* grid, const double x, const double y) {
    return grid_geti(grid, ceili(x), ceili(y));
}

Point* grid_get_relative(const Grid* grid, const Point* point, const Direction direction) {
    switch (direction) {
        case NORTH:
            return grid_geti(grid, point->pos.x, point->pos.y - WIDTH);
        case WEST:
            return grid_geti(grid, point->pos.x - 1, point->pos.y);
        case EAST:
            return grid_geti(grid, point->pos.x + 1, point->pos.y);
        case SOUTH:
            return grid_geti(grid, point->pos.x, point->pos.y + WIDTH);
        default:
            return NULL;
    }
}

// Amount of cells in the middle of the maze, the goal.
#define GOAL_CELL_COUNT 4

// The coordinates of the mid-cells in the maze.
static const Vec2i GOAL_CELLS[GOAL_CELL_COUNT] = {
    {WIDTH / 2 - 1, HEIGHT / 2 - 1},
    {WIDTH / 2, HEIGHT / 2 - 1},
    {WIDTH / 2 - 1, HEIGHT / 2},
    {WIDTH / 2, HEIGHT / 2},
};

/**
 * Returns the smallest value of two numbers.
 * @param x Value one.
 * @param y Value two.
 * @return The smallest value. If x equals y, returns x.
 */
static uint8_t min(const uint8_t x, const uint8_t y) {
    if (x <= y) {
        return x;
    }
    return y;
}

/**
 * Returns the grid distance to the nearest goal.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @return The grid distance to the nearest goal.
 */
static uint8_t grid_distance(const uint8_t x, const uint8_t y) {
    uint8_t umin = UINT8_MAX;

    for (int i = 0; i < GOAL_CELL_COUNT; ++i) {
        const Vec2i* vec2i = &GOAL_CELLS[i];
        const uint8_t distance = abs(x - vec2i->x) + abs(y - vec2i->y);

        umin = min(umin, distance);
    }

    return umin;
}

void grid_init(const Grid* grid, Point* history, uint8_t* history_size) {
    for (uint8_t x = 0; x < WIDTH; ++x) {
        for (uint8_t y = 0; y < HEIGHT; ++y) {
            Point* point = grid_geti(grid, x, y);

            point->pos = (Vec2i){x, y};
            point->distance = grid_distance(x, y);
            for (int j = 0; j < DIRECTION_LENGTH; ++j) {
                point->wall[j] = NULL;
            }
        }
    }

    history[0] = grid->points[0];
    *history_size = *history_size + 1;
}
