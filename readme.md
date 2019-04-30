Symbly
======

Symbly (**Symb**ol Assem**bly**) simple assembly-like language made for fun,
focused on:
- Being easy to read for who's not familiar with assemblies;
- Giving readable and helpful error reports;
- Being RISC;
- Being not complex.

This is actually an experiment. My personal goals:
- Write an compiler for a simple grammar (for fun and practice);
- See if using symbols can really make assembly more readable¹;
- See if it would have any didatic advantages;
- Have practice in CLI/terminal tooling for emulation.

¹: Specially because of instruction formats. I tend to read instructions like
   `add x, y, z` as "add x and y, then store it in z".

In the "end" of this project, I shall add two sections in this readme, one
about the good results, and other about what the bad results.

Currently, I'm planning to implement the VM and Compiler in Rust (to practice
the language).

Registers
---------

| Register   | Purpose                                   |
| ---------- | ----------------------------------------- |
| `zero`     | Read-only register with value 0.          |
| `r0`..`r4` | General purpose (the programmer decides). |
| `a0`..`a3` | Function arguments.                       |
| `sp`       | Stack pointer.                            |
| `ia`       | Current instruction's address.            |
| `ra`       | Last called function's address.           |
| `rv`       | Function's return value.                  |

Instruction Set
---------------

### Planned instructions

These instructions aren't implemented yet, but they will eventually.
In the format examples, `x`, `y`, `z` may be either registers or literals,
while `rx`, `ry` and `rz` are **necessarily** registers.

#### Basic instructions

| Name              | Format                 | Description                                                          |
| ----------------- | ---------------------- | -------------------------------------------------------------------- |
| Set               | `set rx`               | Sets `rx` to 1.                                                      |
| Clear             | `clear rx`             | Sets `rx` to 0.                                                      |

#### Arithmetic instructions

| Name              | Format                 | Description                                                          |
| ----------------- | ---------------------- | -------------------------------------------------------------------- |
| Add               | `rx <- ry + z`         | Adds `ry` and `z` and stores the result into `z`.                    |
| Sub               | `rx <- ry - z`         | Subtracts `ry` and `z` and stores the result into `z`.               |
| Mul               | `rx <- ry * z`         | Multiplies `ry` and `z` and stores the result into `z`.              |
| Div               | `rx <- ry / z`         | Divides `ry` by `z` and stores the result into `z`.                  |
| Left-Shift        | `rx <- ry << z`        | Shifts `ry` left `z` times and stores the result into `rx`.          |
| Right-Shift       | `rx <- ry >> z`        | Shifts `ry` right `z` times and stores the result into `rx`.         |

#### Flow-control instructions

| Name              | Format                 | Description                                                          |
| ----------------- | ---------------------- | -------------------------------------------------------------------- |
| Go-to             | `goto label`           | Sets instruction pointer to `label`.                                 |

#### Conditional instructions

| Name              | Format                 | Description                                                          |
| ----------------- | ---------------------- | -------------------------------------------------------------------- |
| Branch-Equals     | `rx = ry ? goto label` | Sets instruction pointer to `label` if `x` is equals to `y`.         |
| Branch-Not-Equals | `rx = ry ? goto label` | Sets instruction pointer to `label` if `x` is **not** equals to `y`. |
| Set-If-Equals     | `rx = ry ? set rz`     | Sets `rz` to 1 if `rx` is equals to `ry`.                            |

#### Memory instructions

| Name               | Format                 | Description                                                              |
| ------------------ | ---------------------- | ------------------------------------------------------------------------ |
| Load               | `mem[rx] <- ry`        | Stores `y`'s value into memory at address `rx`.                          |
| Load (with offset) | `mem[rx + y] <- rz`    | Stores `rz`'s value into memory at address `rx` (with an offset of `y`). |
| Store              | `rx <- mem[ry]`        | Loads memory value at address `y` and stores into `x`.                   |

Example
-------

Fibonacci function:

```
; Returns n-th number of Fibonacci sequence.
; Args:
;    a0: n (i64)
fib:
    r0 <- 0 ; i
    r1 <- 0 ; x
    r2 <- 1 ; y

    loop:
        ; loop condition
        r0 = a0 ? goto end

        ; x, y = x + y, x
        r3 <- r1 + r2
        r2 <- r1
        r1 <- r3

        ; ++i
        r0 <- r0 + 1
        goto loop
    end:

    ; return x
    rv <- r1
    goto ra
```

Memncpy:

```
; Copies a sequence N bytes from one place to another in memory (little-endian).
; Args:
;     a0: Destiny address.
;     a1: Source address.
;     a2: Number of bytes.
memncpy:
    r0 <- a1 + a2 ; final address (for optimization)
    r1 <- a1      ; current memory address + offset for source
    r4 <- 0       ; i

    copy-loop:
        ; loop condition
        r1 = r0 ? goto end   ; reached final address?

        ; copies one byte
        ; WARN: store/loads only work with words, so we:
        ; - load the word from memory
        ; - change the byte we want
        ; - store the modified word
        r2 <- mem[r1]            ; current word

        r3 <- 0xff00000000000000 ; word mask (all bits from 1st byte are 1)
        r5 <- r4 % 4             ; which byte (in index) to mask
        r3 <- r3 >> r5           ; shifts to correct byte to mask

        r2 <- r2 & r3            ; applies mask

        ; TODO: Complete this section

        ; updates current address
        r1 <- r1 + 1

        ; ++i
        r4 <- r4 + 1
        goto loop
    end:

    ; return
    goto ra
```
