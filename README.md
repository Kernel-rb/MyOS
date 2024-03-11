# MyOS

MyOS is an operating system project written in Rust. Please note that this project is currently under construction and may not be suitable for production use. 🚧

## Project Structure

- **.cargo**: Directory containing cargo configurations.
- **src**: Source code directory.
  - **allocator**: Directory containing allocator-related code.
    - **bump.rs**: Implementation of a bump allocator.
    - **fixed_size_block.rs**: Implementation of a fixed-size block allocator.
    - **linked_list.rs**: Implementation of a linked list allocator.
  - Various Rust source files (.rs) related to your operating system:
    - **gdt.rs**: Global Descriptor Table implementation.
    - **interrupts.rs**: Interrupt handling logic.
    - **lib.rs**: Rust library entry point.
    - **main.rs**: Main entry point of the operating system.
    - **memory.rs**: Memory management logic.
    - **serial.rs**: Serial communication logic.
    - **vga_buffer.rs**: VGA buffer handling logic.
- **tests**: Directory containing test files.
  - **basic_boot.rs**: Test file for basic boot functionality.
  - **should_panic.rs**: Test file for scenarios that should panic.
  - **stack_overflow.rs**: Test file for stack overflow scenarios.
- Other project-related files like **Cargo.toml**, **Cargo.lock**, etc.

## Source Code Overview

- **src/gdt.rs**: File containing the implementation of the Global Descriptor Table.
- **src/interrupts.rs**: File containing interrupt handling logic.
- **src/lib.rs**: Rust library entry point.
- **src/main.rs**: Main entry point of the operating system.
- **src/memory.rs**: File containing memory management logic.
- **src/serial.rs**: File containing serial communication logic.
- **src/vga_buffer.rs**: File containing VGA buffer handling logic.

## Tests

- **tests/basic_boot.rs**: Test file for basic boot functionality.
- **tests/should_panic.rs**: Test file for scenarios that should panic.
- **tests/stack_overflow.rs**: Test file for stack overflow scenarios.

## Contributing

Contributions to MyOS are welcome. If you would like to contribute, please fork the repository and submit a pull request. 👍

## Status

🚧 Work in Progress 🚧

## Helpful Resources

For more insights into operating system development in Rust, consider referring to the following blog: [The Blog of Philipp Oppermann](https://os.phil-opp.com/). 📖
