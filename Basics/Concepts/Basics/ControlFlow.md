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

Loop, like `while(true)`, will not stop until you told it so, to programatically
tell the loop to stop and return a value, we can use `break`:

```rust
fn main(){
    let count = 0;
    let result = loop{
        if count == 2{
            break count; // this will be returned and assign to the variable result
        }
        println!("This is {count} times of iteration.");
    }
    println!("The loop has iterated {result} times"); // 2 times
}
```

Also, for nested loop, we can use loop labels to disambiguate multiple loops:

```rust
//I copied this example from rust book.
fn main() {
    let mut count = 0;
    'counting_up: loop { //the outter loop has label 'counting_up
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //we can use the label to clearly specify which loop we want to break
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
/*
Stdout: 
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
*/
```

## while and for

They are just while and for, note for iterating through elements of collections, we could use ```for...in...```

Nothing else special here.
