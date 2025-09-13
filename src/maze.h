//
// Created by efy on 12/09/25.
//

#ifndef MAZE_H
#define MAZE_H

#include <stdbool.h>
#include <stdint.h>

#include "vec2i.h"

// The maze width.
#define WIDTH 16
// The maze height.
#define HEIGHT 16
// The maze size.
#define SIZE WIDTH * HEIGHT

/**
 * Represents a direction that may contain a wall, as defined in Point.
 */
typedef enum {
    NORTH,
    WEST,
    EAST,
    SOUTH,
    DIRECTION_LENGTH
} Direction;

/**
 * Represents a point in a grid.
 */
typedef struct {
    Vec2i pos;
    uint8_t distance;
    bool wall[4];
} Point;

/**
 * Represents the maze grid.
 */
typedef struct {
    Point* points;
} Grid;

/**
 * Gets a point at a grid coordinate.
 * @param grid The grid.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @return A pointer to the point at x, y.
 */
Point* grid_geti(const Grid* grid, uint8_t x, uint8_t y);

/**
 * Gets a point at a grid coordinate.
 * @param grid The grid.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @return A pointer to the point at x, y.
 */
Point* grid_getd(const Grid* grid, double x, double y);

/**
 * Returns a point relative to the specified point.
 * @param grid The grid.
 * @param point The point.
 * @param direction The direction relative to the point.
 * @return A pointer to the point, or NULL if this point is out of bounds.
 */
Point* grid_get_relative(const Grid* grid, const Point* point, Direction direction);

/**
 * Initializes the grid by calculating the distance to the nearest goal.
 * @param grid The grid pointer.
 * @param history The history pointer.
 * @param history_size The size of the history.
 */
void grid_init(const Grid* grid, Point* history, uint8_t* history_size);

#endif //MAZE_H
