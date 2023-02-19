kernel void m2x2_mul(
  device const float* entries_a,
  device const float* entries_b,
  device float* result
  uint idx [[thread_position_in_grid]]
)
{
  const uint i = (idx / 2) * 2;
  const uint j = idx % 2;
  result[idx] = entries_a[i] * entries_b[j] + entries_a[i + 1] * entries_b[j + 2]
}
