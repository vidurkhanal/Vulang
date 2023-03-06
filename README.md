# Vulang

This is Vulang!! Lightning fast and fault tolerant. The language runs on its pure-rust VM, called VulangVM. The design is highly inspired from Erlang's BEAM and python's C based interpreter.


## Grammar


These are the standard grammatical rules for Vulang's Assembly Interface. By following these rules and structures, one can write and understand programs in Vulang's assembly language.

### Program Structure

A program in our assembly language is composed of instructions.

### Instruction Structure

An instruction in our assembly language is composed of:

- An opcode
- A register
- An integer operand
- A newline character

### Opcode Structure

An opcode in our assembly language is composed of:

- One or more letters in a row
- A space character

### Register Structure

A register in our assembly language is composed of:

- The symbol "$"
- A number
- A space character

### Integer Operand Structure

An integer operand in our assembly language is composed of:

- The symbol "#"
- A number

### Number Structure

A number in our assembly language is composed of:

- The symbols 0-9

### Newline Structure

A newline character in our assembly language is composed of:

- The symbol "\" followed by the symbol "n"


