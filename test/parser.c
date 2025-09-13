//
// Created by efy on 13/09/25.
//

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static void open_file() {
    const FILE* file = fopen("resources/", "r");

    if (file == NULL) {
        perror("Failed to open file");
        exit(EXIT_FAILURE);
    }

    fgets()
}

void parser_get_walls(uint8_t x, uint8_t y, bool* walls) {

}
