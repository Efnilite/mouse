//
// Created by efy on 12/09/25.
//

#ifndef MAZE_H
#define MAZE_H

#include <stdbool.h>
#include <stdint.h>

// The maze width.
#define WIDTH 16
// The maze height.
#define HEIGHT 16

/**
 * Represents a direction that may contain a wall, as defined in Point.
 */
typedef enum {
    NORTH,
    WEST,
    EAST,
    SOUTH
} Direction;

/**
 * Represents a point in a grid.
 */
typedef struct {
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
Point* grid_get(const Grid* grid, uint8_t x, uint8_t y);

/**
 * Initializes the grid by calculating the distance to the nearest goal.
 * @param grid The grid pointer.
 */
void grid_init(const Grid* grid);

#endif //MAZE_H
