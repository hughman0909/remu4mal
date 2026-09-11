# macOS

--- tools installation ---

`$ brew install nasm x86_64-elf-gcc x86_64-elf-binutils`

--- asm to bin ---

`$ nasm -f bin -o sample.bin sample.asm`

--- asm and obj(c) to bin ---

`$ nasm -f elf loader.asm -o loader.o`

`$ x86_64-elf-gcc -32 -nostdlib -fno-asynchronous-unwind-tabl -c sample.c -o sample.o`

`$ x86_64-elf-ld -m elf_i386 --entry=start --oformat=binary -Ttext 0x7c00 -o sample.bin loader.o sample.o`

--- check the bin file inside ---

`$ ndisasm -b 32 sample.bin`

--- GNU asm to bin ---

`$ x86_64-elf-as --32 -o sample.o sample.s`

`$ x86_64-elf-ld -m elf_i386 --oformat=binary -Ttext 0x7c00 -e _start -o sample.bin sample.o`



