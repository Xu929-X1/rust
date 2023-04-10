# Common Programming Concepts - Data Types

## Scalar and Compound

### Scalar Type

There are 4 scalar types in Rust: int, float, bool and char.

- Integer Type

|  Legth  | Signed | Unsigned |                      Comment                      |
| :-----: | :----: | :------: | :-----------------------------------------------: |
|  8-bit  |   i8   |    u8    |                                                   |
| 16-bit  |  i16   |   u16    |                                                   |
| 32-bit  |  i32   |   u32    |                                                   |
| 64-bit  |  i64   |   u64    |                                                   |
| 126-bit |  i128  |   u128   |                                                   |
|  arch   | isize  |  usize   | This is depend on the architecture of the machine |

- Integer Literals

|  Number Literals  |   Example   |
| :---------------: | :---------: |
|      Decimal      |   98_122    |
|        Hex        |    0xff     |
|       Octal       |    0o67     |
|      Binary       | 0b1111_0000 |
| Byte(only for u8) |    b'A'     |

Default type in rust is i32.

-Float Type

Rust has two float types: f32 and f64, the default float type is f64.

- Math Operations

The only thing that needs attention is the result of integer division is
truncated:

```rust
fn main(){
    let res = 5 / -3; //this is -1
}
```

- Boolean

It is boolean, nothing new.

- Char

Char type in rust is 4 bytes (32 bits) in size and represents Unicode scalar
value. That is all we need to know for now.

## Compound Type

There are two primitive compound types: Tuple and Array

- Tuple

Fixed length, not require elements to be the same type, support destructure,
access by period (.):

```rust
fn main(){
    //declare a tuple: 
    let tup: (i32, i64, f32) = (500, 1000, 20.0);
    //destructure: (here we think of it as a vec3)
    let (x, y, z) = tup;
    //or access value with period
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
}
```

- Array

Fixed length, require elements to be the same type, useful when wish to allocate
data in a stack instead of a heap,

Syntax:

```rust
fn main(){
    //declaration: 
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a: [3; 5]; //[3, 3, 3, 3, 3]

    //access value:
    let first = a[0];
}
```
