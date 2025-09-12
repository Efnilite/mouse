//
// Created by efy on 12/09/25.
//

#ifndef VEC2D_H
#define VEC2D_H

/**
 * Represents a vector.
 */
typedef struct {
    double x;
    double y;
} Vec2d;

/**
 * Gets the length of a vec2d.
 * @param vec2d The vector.
 * @return The length of the vector.
 */
inline double vec2d_length(const Vec2d* vec2d);

#endif //VEC2D_H
