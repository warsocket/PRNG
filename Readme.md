# Compilation #

### nasm ###
nasm -f elf64 prng.nasm && ld -o  prng prng.o

### rust ###
cargo build --release

### python ###
(Just run it)
