//
// Created by efy on 12/09/25.
//

#ifndef MAZE_H
#define MAZE_H

#include <stdint.h>

// The maze width.
#define WIDTH 16
// The maze height.
#define HEIGHT 16

typedef struct {
    uint8_t distance;
} Point;

typedef struct {
    Point points[HEIGHT * WIDTH];
} Grid;

/**
 * Gets a point at a grid coordinate.
 * @param grid The grid.
 * @param x The x coordinate.
 * @param y The y coordinate.
 * @return A pointer to the point at x, y.
 */
inline Point* grid_get(Grid* grid, uint8_t x, uint8_t y);

/**
 * Initializes the grid by calculating the distance to the nearest goal.
 * @param grid The grid pointer.
 */
void grid_init(Grid* grid);

#endif //MAZE_H
