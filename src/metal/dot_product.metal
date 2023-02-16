kernel void dot_product(
  device const uint* inA,
  device const uint* inB,
  device uint* result,
  uint index [[thread_position_in_grid]])
{
  result[index] = inA[index] * inB[index];
}
