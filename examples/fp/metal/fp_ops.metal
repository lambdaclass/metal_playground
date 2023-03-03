
#include "fp.h.metal"

kernel void sum(
  device const uint32_t* _a,
  device const uint32_t* _b,
  device uint32_t* result,
  uint2 grid_size [[grid_size]], // matrices size (row and col)
  uint2 pos [[thread_position_in_grid]]
)
{
  Fp a = *_a;
  Fp b = *_b;

  result[0] = (a + b).asUInt32();
}

kernel void mul(
  device const uint32_t& _a,
  device const uint32_t& _b,
  device uint32_t& result,
  uint2 grid_size [[grid_size]], // matrices size (row and col)
  uint2 pos [[thread_position_in_grid]]
)
{
  Fp a = _a;
  Fp b = _b;

  result = (a * b).asUInt32();
}
