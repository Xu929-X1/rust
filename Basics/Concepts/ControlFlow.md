# Common Programming Concepts - Control flow

This part is for me to get familiar with the syntax.

- If statement

Note the condition must be `bool`, non-boolean value will not be casted like JS.

```rust
fn main(){
    let x = 5; 
    if x == 5 {
        println!("The number is 5!");
    }else if x < 5 {
        println!("The number is smaller than 5!");
    }else if x > 5{
        println!("The number is greater than 5!");
    }else {
        println!("I have no idea what the number is");
    }
}
```

Also, we can use if **expression** to selectively assign a value to a variable:

```rust
fn main(){
    let condition = true;
    let number = if condition {5} else {6}; //note here, in the brackets are expressions

    //Another thing to note here is that the types in the branches must be the same
    let another_number = if condition {5} else {"Six"}; //this will not compile, as the variable has 2 possible types. Although the type in runtime is singular, compiler is not able to determine that

    println!("The value of the number is {number}");//5
}
```

## Loop

Instead of `while` and `for` loop, rust also provides a `loop` loop:

- Loop

Loop, like `while(true)`, will not stop until you told it so:
