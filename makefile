compile_metal:
	xcrun -sdk macosx metal -c examples/${EXAMPLE}/metal/${EXAMPLE}.metal -o examples/${EXAMPLE}/metal/${EXAMPLE}.air
	xcrun -sdk macosx metallib examples/${EXAMPLE}/metal/${EXAMPLE}.air -o examples/${EXAMPLE}/metal/${EXAMPLE}.metallib

