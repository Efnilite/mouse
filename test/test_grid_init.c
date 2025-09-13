//
// Created by efy on 12/09/25.
//

#include <assert.h>
#include <stdlib.h>

#include "../src/maze.h"

static Point points[SIZE];
static Grid grid = {points};

static Point history[SIZE];
static uint8_t history_size = 0;

int main(void) {
    grid_init(&grid, history, &history_size);

    assert(grid_geti(&grid, 0, 0)->distance == 14);
    assert(grid_geti(&grid, 1, 0)->distance == 13);
    assert(grid_geti(&grid, 2, 0)->distance == 12);
    assert(grid_geti(&grid, 3, 0)->distance == 11);
    assert(grid_geti(&grid, 4, 0)->distance == 10);
    assert(grid_geti(&grid, 5, 0)->distance == 9);
    assert(grid_geti(&grid, 6, 0)->distance == 8);
    assert(grid_geti(&grid, 7, 0)->distance == 7);
    assert(grid_geti(&grid, 8, 0)->distance == 7);
    assert(grid_geti(&grid, 9, 0)->distance == 8);

    assert(grid_geti(&grid, 8, 1)->distance == 6);
    assert(grid_geti(&grid, 8, 2)->distance == 5);
    assert(grid_geti(&grid, 8, 3)->distance == 4);
    assert(grid_geti(&grid, 8, 4)->distance == 3);
    assert(grid_geti(&grid, 8, 5)->distance == 2);
    assert(grid_geti(&grid, 8, 6)->distance == 1);
    assert(grid_geti(&grid, 8, 7)->distance == 0);

    assert(grid_geti(&grid, 0, 8)->distance == 7);
    assert(grid_geti(&grid, 1, 8)->distance == 6);
    assert(grid_geti(&grid, 2, 8)->distance == 5);
    assert(grid_geti(&grid, 3, 8)->distance == 4);
    assert(grid_geti(&grid, 4, 8)->distance == 3);
    assert(grid_geti(&grid, 5, 8)->distance == 2);
    assert(grid_geti(&grid, 6, 8)->distance == 1);
    assert(grid_geti(&grid, 7, 8)->distance == 0);

    return EXIT_SUCCESS;
}
