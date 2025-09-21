#include <stdio.h>
#include <stdlib.h>

#include "maze.h"
#include "path.h"
#include "vec2i.h"
#include "parser.h"

static Point points[SIZE];
static Grid grid = {points};

static uint8_t history_size = 0;
static Point history[SIZE];

static Vec2i pos = {0, 0};

int main(void) {
    open_maze_file();

    grid_init(&grid, history, &history_size);

    while (true) {
        const Point* next = grid_calculate_path(&grid, history, history_size, &pos);

        if (next->distance == 0) {
            break;
        }

        if (history_size >= SIZE) {
            return EXIT_FAILURE;
        }

        history[history_size] = *next;
        history_size++;
    }

    close_maze_file();

    return EXIT_SUCCESS;
}
