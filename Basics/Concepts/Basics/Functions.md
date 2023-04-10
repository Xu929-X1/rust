# Common Programming Concepts - Functions

- main function is the entry point.

- use snake case for naming

## Statements and Expressions

The key difference between a statement and an expression is, a statement are
instructions that perform some actions without returning a value:

```rust
fn main(){
    let y = 6; //this is a statement
}
```

An expression evaluates the resultant value:

```rust
fn main(){
    let y = {
        let x = 3;
        x + 1 //this is an expression, an expression does not include end semicolon
    }; 

    println!("The final value of y is {y}");//4
}
```

## Return Value

- To declare the return type of a function, use `->`, in a function, we could
  use `return` keyword to explicitly return a value, or the last expression
  implicitly.

TBH this is a bit strange looking but:

```rust
fn Five()->i32{
    5 //this does not have a end semicolon, so it is the last expression, which is returned implicitly
}

fn main(){
    let x = Five();

    println!("The value of x is: {x}");
}
```
