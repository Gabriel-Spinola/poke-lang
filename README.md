# Poke Lang

poke-lang is a bytecode interpreted language inspired by Lua, Gleam, and the "Crafting Interpreters" book. It is designed to resemble a statically typed Lua and serve as a scripting language for handling small tasks.

## References
- [Crafting Interpreters](https://craftinginterpreters.com/chunks-of-bytecode.html)
- [Building Lua in Rust](https://github.com/WuBingzheng/build-lua-in-rust)
- [Gleam Compiler Core](https://github.com/gleam-lang/gleam/blob/main/compiler-core/)

## Syntax
```lua
function sum(int a, int b) -> int
  if a >= 2 then
    return b
  end

  return a + b
end

mut int a = 2
-- 2
print a

-- function mutate(mut int a)    |    also valid
function mutate(mut int a) -> void
  a = 5 + 6
end

mutate a

-- 11
print a
```
The project is designed to be used for interpreting bytecode and scripting small tasks. More details on usage and features can be found in the codebase.
