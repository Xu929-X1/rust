# Common Programming Concepts - Variables 

## Varaibles

1. Inmmutability

- Variables in Rust are immutable by default, but we can use:

```rust
let mut var = 10;
var = 20
```

to mutate the variables

2. Constants

Constants does not allow using `mut`. And the type of a constant must be
annotated, like:

```rust
const MINITES_IN_FIVE_HOURS : u32 = 5 * 60;
```

A convention is that for constant, we use all uppercase names.

3. Shadowing

If you declare another variable with the same name as a already declared
variable, the already declared variable will be shadowed.

```rust
fn main(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of the inner scope is {}", x);//12
    }
    println!("The value of the outter scope is {}", x);//6
}
```

The key difference between shadowing and mutable variable declaration is that the shadowing, since it is declaring a new variable, is able to change the variable's type: 

```rust
fn main(){
    //this is ok
    let length = "length";
    let length = length.len();

    //this will cause type error, as length was previously a string type, and now tring to assign a number type to it
    let mut length = "length";
    length = length.len(); //error
}
```
