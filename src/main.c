#include <stdio.h>
#include <stdlib.h>

#include "maze.h"

int main(int argc, char* argv[]) {
    Point points[WIDTH * HEIGHT];
    Grid grid = {
        .points = points
    };

    grid_init(&grid);

    return EXIT_SUCCESS;
}
