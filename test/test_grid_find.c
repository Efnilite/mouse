//
// Created by efy on 12/09/25.
//

#include <stdlib.h>
#include <assert.h>

#include "../src/maze.h"
#include "../src/vec2d.h"

Point points[WIDTH * HEIGHT];
Grid grid = {points};

Vec2d position = {0, 0};
Vec2d speed = {0, 0};

int main(void) {
    grid_init(&grid);

    return EXIT_SUCCESS;
}
