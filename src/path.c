//
// Created by efy on 13/09/25.
//

#include "path.h"

#include <stdio.h>

Point* grid_calculate_path(const Grid* grid, const Point* history, const uint8_t history_size, const Vec2i* pos) {
    uint8_t smallest_distance = UINT8_MAX;
    Point* smallest = NULL;

    for (int i = 0; i < SIZE; ++i) {
        if (i >= history_size) {
            break;
        }

        const Point* point = &history[i];

        uint8_t wall_count = 0;
        for (int j = 0; j < DIRECTION_LENGTH; ++j) {
            if (point->wall[j]) {
                wall_count++;
            }
        }

        // only one way so skip distance calculation
        if (wall_count == 3) {
            continue;
        }

        for (int j = 0; j < DIRECTION_LENGTH; ++j) {
            if (point->wall[j]) {
                continue;
            }

            Point* relative = grid_get_relative(grid, point, (Direction) j);

            if (relative == NULL) {
                continue;
            }
            printf("r %d,%d with d %d\n", relative->pos.x, relative->pos.y, relative->distance);

            if (relative->distance < smallest_distance) {
                printf("updated smallest to %d,%d with d %d\n", relative->pos.x, relative->pos.y, relative->distance);
                smallest = relative;
                smallest_distance = relative->distance;
            }
        }
    }

    return smallest;
}
