# String in Depth

In Rust, the string datatype is more complex than what I thought. Strings are implemented as collections of bytes. In the core language, there is only string type, string slice, aka ```str```. String literals, for example, are stored in the program binary and are thus string slices.

The ```String``` type is from the standard library. It is growable, mutable and owned UTF-8 encoded string type. 