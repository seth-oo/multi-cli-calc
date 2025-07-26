# Multi-Purpose CLI Calculator

A simple and powerful command-line calculator written in pure Rust using **procedural programming only**. This tool can perform multiple operations like addition, subtraction, multiplication, and more.

##  Features

- Basic arithmetic (add, subtract, multiply, divide)
- Floating-point support
- Multi-step operations
- Simple terminal UI with input prompts
- Handles invalid inputs
- Extendable (supports adding new operations easily)

## Getting Started

### Prerequisites

- Rust installed on your system  
  (Install from [https://rustup.rs](https://rustup.rs))

### How to Run

1. Clone the repository:

   ```bash
   git clone https://github.com/seth-oo/multi-cli-calc.git
   cd multi-cli-calc
   rustc calculator.rs -o calculator
   ./calculator
   ```

2. Usage Example
   ```bash
   === Advanced Procedural CLI Calculator ===
   Enter PIN to access calculator:
   PIN : SETH
   Authentication successful!
   Welcome! Type 'help' or 'h' for available commands.

   > h
   === Advanced Calculator Help ===
   Basic Operations:
    2 + 3, 5 - 2, 4 * 6, 8 / 2, 2 ^ 3, 17 % 5

   Scientific Functions:
   sqrt(16), sin(30), cos(60), tan(45), log(100), ln(2.718)
   abs(-5), fact(5)

   Complex Numbers:
   complex 3 4

   Bitwise Operations:
    AND 12 10, OR 12 10, XOR 12 10, NOT 12
    SHL 5 2, SHR 20 2 (shift left/right)

   Combinatorics:
     nCr 5 2, nPr 5 2

   Angle Conversion:
     deg2rad 180, rad2deg 3.14159

   Equation Solving:
    quad 1 -3 2 (ax²+bx+c=0), solve 2x+3=7

   Statistics:
     mean 1 2 3 4 5, sum 1 2 3 4 5

   Memory & Storage:
     store tax = 0.075, recall tax, mem (show all)
     $1, $2, $3 (last 3 results)

   Fractions:
     frac 3 4, gcd 12 8

   Random & Primes:
     rand, isprime 23

   Command Shortcuts:
     h=help, c=clear, q=quit, v=vars, s=save, l=load
   Multiple commands: x=5; y=x+3; print y

   Other Commands:
    settings, history, dump, vars, constants

   > _

   ```

### Note
   This calculator was built entirely without OOP or functions—**just pure procedural programming**—to deepen understanding of low-level control and structure.

   **Default PIN:** SETH
   You’ll need this PIN to access or test the CLI app. You can change the default PIN directly in the code if desired.

**Disclaimer:**
  You might encounter a warning when compiling the program. This warning will not prevent the program from running or functioning correctly as a real CLI app—you can still fully test and use it.
   

