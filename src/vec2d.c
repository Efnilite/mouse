//
// Created by efy on 12/09/25.
//

#include "vec2d.h"

#include <math.h>

double vec2d_length(const Vec2d* vec2d) {
    return sqrt(vec2d->x * vec2d->x + vec2d->y * vec2d->y);
}
