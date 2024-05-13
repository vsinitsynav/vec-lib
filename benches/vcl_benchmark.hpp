#pragma once

#include <vector>
#include <cstdint>

namespace bench {

    void MulVec8iVCL(const std::int8_t *vec1, const std::int8_t *vec2, std::int8_t *result, std::size_t n);

}  // namespace bench

