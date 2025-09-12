#include <stdio.h>
#include <stdlib.h>

#include "maze.h"

int main(void) {
    Point points[WIDTH * HEIGHT];
    Grid grid = {
        .points = points
    };

    grid_init(&grid);

    return EXIT_SUCCESS;
}
