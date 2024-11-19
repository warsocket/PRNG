#!/usr/bin/env python3
import sys
import struct
from signal import signal, SIGPIPE, SIG_DFL
signal(SIGPIPE,SIG_DFL) 

LARGEST64PRIME = 18446744073709551557

def rotr(num, shift, bitsize=64):
	assert(shift < 128)
	top = num << 128 >> shift & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
	return num >> shift | top >> (128-bitsize)


# data = 0x0102030405060708
data = 0x80876363d18c506d

bits = [
    0xFFFFFFFFFFFFFFFF,
    0xAAAAAAAAAAAAAAAA,
    0xCCCCCCCCCCCCCCCC,
    0xF0F0F0F0F0F0F0F0,
    0xFF00FF00FF00FF00,
    0xFFFF0000FFFF0000,
    0xFFFFFFFF00000000,
]
bits7 = 0xFFFFFFFFFFFFFFFF0000000000000000


# for _ in range(2):
while True:

	n64 = data

	for i,b in enumerate(bits):
		mask = (1<<i)-1
		shift = n64 & mask
		bits[i] = rotr(bits[i], shift)
		data ^= bits[i]
		n64 >>= i

		if (i):
			mask = 0x3f #lower 6 bits high
			shift = n64 & mask
			data = rotr(data, shift)
			n64 >>= 6


	mask = (1<<i)-1
	shift = n64 & mask
	bits7 = rotr(bits7, shift, 128)
	data ^= (bits7 & 0xFFFFFFFFFFFFFFFF)

	data = data*LARGEST64PRIME & 0xFFFFFFFFFFFFFFFF

	sys.stdout.buffer.write(struct.pack(">Q", data))





