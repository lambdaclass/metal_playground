#include <metal_stdlib>
#include <metal_simdgroup_matrix>

kernel void build_matrix(
  device const float** in_a, // array of 2x2 blocks (which will be arrays of floats)
  device const float** in_b, // same as above
  device const uint n,       // such that n*n is the length of these previous arrays
  device float** result,     // array of 2x2 blocks
  uint2 pos [[threadgroup_position_in_grid]]
)
{
  simdgroup_float2x2 sg_a;
  simdgroup_float2x2 sg_b;
  simdgroup_float2x2 sg_result; // will accumulate results

  for (uint i = 0; i < n*n; i++) {
    simdgroup_load(sg_a, in_a[i + pos.y * n]);
    simdgroup_load(sg_b, in_b[i * n + pos.x]);

    simdgroup_multiply_accumulate(sg_result, sg_a, sg_b, sg_result);
    // equivalent to sg_result = sg_a * sg_b + sg_result
  }

  simdgroup_store(sg_result, result[pos.y * n + pos.x]);
}
