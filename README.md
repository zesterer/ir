IR
============
*Name to be determined.*

A simple, flexible, and portable SSA IR for compiled programming languages.

## Example

All branches are known as "blocks". Blocks are like functions by themselves, making them more powerful than an ordinary local label. 
Blocks have dependencies on other blocks for their functionalities. IR will optimize based on the "shared functionality" of blocks 
when deciding inlining and procedure calls.

Blocks receive any number of inputs through their parameters, but cannot return values. Instead of returning values, blocks mutate their inputs.

The following is a "hello world" example:

```
BLOCK (impure) _main (argc: I32, argv: &&U8)
    s0 = "Hello, world!"
    extern_call printf s0 ; C FFI "printf"
    branch _loop(0) ; a jump / procedure call

BLOCK (impure) _loop (i: I32)
    s0 = "%i"
    extern_call printf s0 i ; printf("%i", i)
    reloop = i < 10 ; condition evaluates to True or False
    branch_if reloop _loop(i) _end ; if (reloop) _loop(i) else _end

BLOCK (impure) _end ()
    branch_end 0 ; return 0
```

`_main` is the entry point of the program, and has the same function signature as the C entry:

```c
int main(int argc, char const* argv[])
```

Note that every block has to end with some kind of `branch_` statement, whether it is a jump, conditional,
or end of process signal.
