#include "vcl_benchmark.hpp"
#include "version2/vectorclass.h"

void bench::MulVec8iVCL(const std::int8_t *vec1, const std::int8_t *vec2, std::int8_t *result, std::size_t n) {
    Vec16c vcl_vec1;
    Vec16c vcl_vec2;
    Vec16c vcl_multiplied;

    size_t i = 0;

    for (; i + 16 < n; i += 16) {
        vcl_vec1.load(vec1 + i);
        vcl_vec2.load(vec2 + i);
        vcl_multiplied = vcl_vec1 * vcl_vec2;
        vcl_multiplied.store(result + i);
    }

    if (i < n) {
        vcl_vec1.load_partial(n - i, vec1 + i);
        vcl_vec2.load_partial(n - i, vec2 + i);
        vcl_multiplied = vcl_vec1 * vcl_vec2;
        vcl_multiplied.store_partial(n - i, result + i);
    }
}

