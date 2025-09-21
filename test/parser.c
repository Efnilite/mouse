//
// Created by efy on 13/09/25.
//

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../src/maze.h"

#define TEST_MAZE_WIDTH 27
#define TEST_MAZE_HEIGHT 14

FILE* file;
char chars[TEST_MAZE_WIDTH * TEST_MAZE_HEIGHT];

void open_maze_file() {
    file = fopen("test/resources/maze", "r");

    if (file == NULL) {
        perror("Failed to open maze file");
        exit(EXIT_FAILURE);
    }

    char line[TEST_MAZE_WIDTH + 2];

    for (int i = 0; i < TEST_MAZE_HEIGHT; ++i) {
        fgets(line, sizeof(line), file);

        for (int j = 0; j < TEST_MAZE_WIDTH; ++j) {
            chars[TEST_MAZE_HEIGHT * i + j] = line[j];
        }
    }
}

void close_maze_file() {
    if (fclose(file) < 0) {
        perror("Failed to close file");
        exit(EXIT_FAILURE);
    }
}

void parser_get_walls(const uint8_t x, const uint8_t y, bool* walls) {
    const uint8_t idx = ((y + 1) * 4 - 3) * TEST_MAZE_WIDTH + ((x + 1) * 4 - 3);

    walls[WEST] = chars[idx + 1] == '#';
    walls[EAST] = chars[idx - 1] == '#';
    walls[NORTH] = chars[idx - TEST_MAZE_WIDTH] == '#';
    walls[SOUTH] = chars[idx + TEST_MAZE_WIDTH] == '#';
}
