#include <metal_stdlib>
using namespace metal;

kernel void assign(
  device uint* buffer,
  uint index [[ thread_position_in_grid ]]
)
{
  buffer[index] = 1;
}
