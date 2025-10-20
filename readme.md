# Pangrams - October 2025 Challenge

From a list of json string entities, determine which are perfect or inperfect pangrams

## What is a Pangram

A pangram is a seqence of characters that combined represent all letters of the alphabet. A perfect Pangram is one which uses all letters once only

### Side notes

1. During each iteration, the string is not stripped of any illegal characters - only mutated to the lowercase instance
2. Serde is used to read the string into a vec<Value> - This is then processed. Some optimizations may be formcoming by first cleaning the list and moving to string beforehand
3. A hashmap is used to maintain the distribution of each 'char' - This may represent a performance bottleneck which can be improved upon

## Challenge

Part of the Rust developers challenge

