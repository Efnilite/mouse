//
// Created by efy on 12/09/25.
//

#ifndef VEC2I_H
#define VEC2I_H

#include <stdint.h>

/**
 * Represents a vector.
 */
typedef struct {
    uint8_t x;
    uint8_t y;
} Vec2i;

/**
 * Gets the length of a vec2i.
 * @param vec2i The vector.
 * @return The length of the vector.
 */
inline double vec2i_length(const Vec2i* vec2i);

#endif //VEC2I_H
