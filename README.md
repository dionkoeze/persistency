# Persistency
The multiplicative persistency of a number is the number of times the following operation can be applied until you are left with a one digit number. The operation is multiplying the individual digits of a number together. For example: 39 -> 27 -> 14 -> 4, so 39 has a multiplicative persistency of 3.

Let's introduce some notation. The digit multiplication procedure we call `f`, so that in our example `f(39) = 27`. The persistency of a number we call `p`, so that in our example `p(39) = 3`. 

## Current record
At the moment the record is a persistency of 11 for the number `277,777,788,888,899`. It is conjectured this is the maximum, and this is checked for numbers up to `10^233`. But we're going to look for a new record anyway. 

The steps for this record number are:
```
277777788888899
4996238671872
438939648
4478976
338688
27648
2688
768
336
54
20
0
```

## Some notes on persistency

#### Combining digits
From a number you can make another number with the same persistency. Any two digits that multiplied together yield a digit below 10 can be a replacement. For example `227` and `47` have the same persistency, since `2*2 = 4`.

#### Prime factorization
The prime factorization of a number `n` gives the possible digits another number `m` can have to result in `n` after applying digit multiplication. For example, take `n = 2*2*3*7 = 84`, then there is a set of numbers that consists of the factors `{2,2,3,7}` and their combinations with all numbers `m` that result `n`.

This set of numbers contains all permutations of `{2,2,3,7}` and all permutations of sets obtained from combining digits. For example all permutations of `{3,4,7}`.

#### Divisibility of numbers
There are some tests to see if a number is divisble by some numbers. Here we list the tests we need.

**2** The last digit is a multiple of 2.

**3** The sum of the digits is divisible by 3.

**5** The last digit is a multiple of 5

**7** The reverse alternating sum of blocks of three digits is divisible by 7. (Example: 6,976,984 => 984 - 976 + 006 = 14)

## A searching strategy
We'll search for new numbers in a chain of persistency from small to large numbers. That is, we start with some number `n` and determine which numbers `m` exist such that `f(m) = n`. Now if `m` has a prime factorization consisting only of 2, 3, 5 and 7 it's a candidate for a larger number in the chain. The prime factorization needs to not contain larger primes, since the number can then not be written as the product of digits (7 is the largest prime below 10).


## Implementation

#### Arbitrary precision numbers
A number is stored as a vector of digits. All operations on the number are done on this vector. 