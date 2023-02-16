kernel void dot_product(
  device const float* inA,
  device const float* inB,
  device float* result,
  uint index [[thread_position_in_grid]])
{
  result[index] = inA[index] * inB[index];
}
