[[kernel]]
void dot_product(
  constant uint *inA [[buffer(0)]],
  constant uint *inB [[buffer(1)]],
  device uint *result [[buffer(2)]],
  uint index [[thread_position_in_grid]])
{
  result[index] = inA[index] * inB[index];
}
