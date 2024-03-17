## Poke-lang
discoraged mutability 

## Defining variables
mut int a = 10
int b = 2

## functions
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

## STD Lib
require poke

poke:print "Torta de maçã"

## keyword
let ⎓ ᔑᓭᓭ
ᔑᓭᓭ cu = 