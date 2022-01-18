# FantASM

A fantasy assembly language written at 3 AM

## Features

* 64 8-bit registers.

## Default Environment

FantASM's example environment will be as follows:

* The address space will be 65536 bytes long.
* The lower half of the address space will be mapped to ROM (The file you run in FantASM)
* The upper half of the address space will be mapped to RAM and control registers.
  * Addresses from 0x8000 to 0x807F are special registers.
    * 0x8000 is the console register. Reading from it will block the program until the user writes a character, and writing to it will block and print to the console.
    * All other registers are unused in the current implementation.
  * Addresses from 0x8080 to 0xFFFF are RAM, and can be freely modified.

## Instructions

Opcodes are 1 byte long, and the length of their arguments is dependent on the opcode.

Instructions are specified like this:

```
0x00 // Opcode
ARGx1 // Argument mnemonic, length in bytes
...
```

* Memory
  * LD - 0x00 FROMx8 TOx1 - Load an address from memory to a register.
  * SV - 0x01 FROMx1 TOx8 - Save an address from a register to memory.
  * MV - 0x02 FROMx1 TOx1 - Copy one register to another, zeroing or truncating if necessary.
  * CP - 0x03 FROMx8 TOx8 SIZEx1 STEPx1 - Copy SIZE bytes from an address to an address, moving the target address by STEP for each byte. STEP is a signed integer.
* Arithmetic
  * INC - 0x04 TGTx1 - Increment a register.
  * DEC - 0x05 TGTx1 - Decrement a register.
  * ADD - 0x06 FROMx1 TOx1 - Add a register to another.
  * ADS - 0x07 FROMx1 TOx1 - Add a register to another, treating the registers as signed integers.
  * ADF - 0x08 FROMx1 TOx1 - Add a register to another, treating the registers as floating point numbers. Only applicable to 32-bit and 64-bit registers, otherwise a no-op.
  * MUL - 0x09 FROMx1 TOx1 - Multiply a register by another.
  * MLS - 0x0A FROMx1 TOx1 - Multiply a register by another, treating the registers as signed integers.
  * MLF - 0x0B FROMx1 TOx1 - Multiply a register by another, treating the registers as floating point numbers. Only applicable to 32-bit and 64-bit registers, otherwise a no-op.
  * DIV - 0x0C FROMx1 TOx1 - Divide a register by another.
  * DVS - 0x0D FROMx1 TOx1 - Divide a register by another, treating the registers as signed integers.
  * DVF - 0x0E FROMx1 TOx1 - Divide a register by another, treating the registers as floating point numbers. Only applicable to 32-bit and 64-bit registers, otherwise a no-op.
  * SUB - 0x0F FROMx1 TOx1 - Subtract a register from another.
  * SBS - 0x10 FROMx1 TOx1 - Subtract a register from another, treating the registers as signed integers.
  * SBF - 0x11 FROMx1 TOx1 - Subtract a register from another, treating the registers as floating point numbers. Only applicable to 32-bit and 64-bit registers, otherwise a no-op.
  * FLG - 0x12 TOx1 - Write the arithmetic flags to a register.
    * The lowest bit is the carry flag.
    * The next bit is the overflow flag.
    * The higher bits are undefined, please don't read them.
  * CLR - 0x13 - Clear the arithmetic flags.
* Bitwise
  * SR - 0x14 TGTx1 AMTx1 - Shift a register right by AMT.
  * SL - 0x15 TGTx1 AMTx1 - Shift a register left by AMT.
  * OR - 0x16 FROMx1 TOx1 - Bitwise OR a register with another, writing the result to TO.
  * AND - 0x17 FROMx1 TOx1 - Bitwise AND a register with another, writing the result to TO.
  * XOR - 0x18 FROMx1 TOx1 - Bitwise XOR a register with another, writing the result to TO.
  * NOT - 0x19 TGTx1 - Bitwise NOT a register, writing the result to TO.
* Jumps
  * JMZ - 0x1A FROMx1 TGTx8 - Jump to TGT if FROM is zero.
  * JNZ - 0x1B FROMx1 TGTx8 - Jump to TGT if FROM is not zero.
  * JMP - 0x1C TGTx8 - Jump to TGT.
* Stack
	* PUSH - 0x1D FROMx1 - Push a value to the stack.
	* POP - 0x1E TOx1 - Pop a value from the stack.
	* CAL - 0x1F TGTx8 - Push the current address to the stack, and jump to TGT.
	* RET - 0x20 - Pop the last 8 bytes from the stack, and jump to it.