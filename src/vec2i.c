//
// Created by efy on 12/09/25.
//

#include "vec2i.h"

#include <math.h>

inline double vec2i_length(const Vec2i* vec2i) {
    return sqrt(vec2i->x * vec2i->x + vec2i->y * vec2i->y);
}
