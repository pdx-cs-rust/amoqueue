# amoqueue: Amortized-constant time queue of references
Bart Massey

This data structure is based on an old trick for maintaining
a queue "lazily" using two lists to provide amortized
constant-time access. This data structure is neither
particularly performant nor particularly convenient, and is
intended primarily as an exercise.

See the rustdoc for documentation.
