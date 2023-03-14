
#include "fp.h.metal"

kernel void sum(
  device const uint32_t* _a,
  device const uint32_t* _b,
  device uint32_t* result,
  uint2 grid_size [[grid_size]], // matrices size (row and col)
  uint2 pos [[thread_position_in_grid]]
)
{
  const uint32_t P = 15 * (uint32_t(1) << 27) + 1;
  Fp <P, 0x88000001, 1172168163> a = *_a;
  Fp <P, 0x88000001, 1172168163> b = *_b;

  result[0] = (a + b).asUInt32();
}

  // static constant uint32_t P = 15 * (uint32_t(1) << 27) + 1;
  // static constant uint32_t M = 0x88000001;
  // static constant uint32_t R2 = 1172168163;

kernel void mul(
  device const uint32_t& _a,
  device const uint32_t& _b,
  device uint32_t& result,
  uint2 grid_size [[grid_size]], // matrices size (row and col)
  uint2 pos [[thread_position_in_grid]]
)
{
  const uint32_t P = 15 * (uint32_t(1) << 27) + 1;
  Fp <P, 0x88000001, 1172168163> a = _a;
  Fp <P, 0x88000001, 1172168163> b = _b;

  result = (a * b).asUInt32();
}
