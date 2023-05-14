# Struct Example

Let us follow what the Rust Book proposes: write a program to calculate the area of a rectangle.

First, we use plain variables for width, height, etc. Later, we will use ```struct``` and see what value it brings.

```rust
fn main(){
    let width = 10;
    let height = 20;

    let area = get_area(width, height);
    println!("The area is {} sq meters", area);
}

fn get_area(width: u32, height:u32)->u32{
    width * height
}
```

But if we evaluate the quality of the code, we'll find that, first, the variable ```height``` and ```width``` are not necessarily relevant, and the function ```get_area``` is calculating an area for one rectangle but has 2 parameters. In this case, to optimize the code to make it more readable and structured, we can use ```struct```.

The first plan that come in mind is using tuple struct, we can refactor our code like this:

```rust
fn main(){
    let rec1 = (10, 20);

    let area = get_area(rec1);
    println!("The area is {} sq meters!", area);
}

fn get_area(dimension: (u32, u32))->u32{
    dimension.0 * dimension.1
}
```

In this way, the function ```get_area``` only takes one parameter and the width and height is stored in the tuple. But we are not clear which one is the width and the height at this time, so we need a more clear way to specify this. And the biggest flaw of this is, the function ```get_area``` is only for rectangle, we wish to somehow reflect it in the parameter:

```rust
struct Rectangle {
    width: u32,
    height: u32
}

fn main(){
    let rec1 = Rectangle{
        width: 10,
        height: 20
    };

    let area = get_area(rec1);
    println!("The area is {} sq meters!", area);
}

fn get_area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
```

This now looks more clear than the previous versions. We know the value of the width and the height, and we could use only one parameter to achieve the goal. In detail, the parameter ```rectangle``` is a immutable borrow of the struct instance. In this way, the main function retains the ownership so we could continue using the ```rec1``` instance.

## Print it?

In programming, a common way to debug is print a variable out so we can get a glimpse what it is, but if we do that for ```rec1```, it will not compile.

```diff
- The Following code will not compile!
```

```rust
struct Rectangle {
    width: u32,
    height: u32
}

fn main(){
    let rec1 = Rectangle{
        width: 10,
        height: 20
    };

    println!("{}", rec1);
}
```

This is because ```struct``` does not have implementation ```std::fmt::Display```. But the compiler gives out a solution:

```stdout
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

We could try to use the specifier ```:?``` inside the curly brackets:

```rust
println!({:?}, rec1);
```

This will use a trait called ```Debug```, but we still get an error:

```stdout
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust does not have functionality to print out debug info, but provide a opt-in to make the functionality available for our struct, we can achieve it by providing an outer attribute ```#[derive(Debug)] before the struct definition:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

}
```

Another way to print out debug information is using the ```dbg!``` macro, which will take the ownership from what is passed in, prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, **and returns ownership of the value.** (```println!``` only take the reference) In addition, ```dbg!``` prints to stderr, as oppose to ```println!```, which prints to stdout. Here is the example of how to use ```dbg!```:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32
}

fn main(){
    let rec1 = Rectangle{
        width: 10,
        height: 20
    }

    dgb!(&rec1);
}
```

Still, there is one problem with the current implementation, which is, the function to calculate the area is only for rectangle, we want to associate it with the rectangle ```struct``` to make our code more clear. We could use method to do so, unlike function, methods are defined within the context of the ```struct```. The first parameter is always ```self```, which represent the instance of the ```struct``` the method is being called on. (Like ```this``` in Javascript)

## Method Syntax

### Definition

Let us define a ```get_area``` function base on the ```Rectangle``` instance context:

```rust
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn get_area(&self)->u32{
        &self.width * &self.height
    }
}

fn main(){
    let rec1 = Rectangle{
        width: 10,
        height:20
    }

    println!("The area of the rectangle is {}", rec1.get_area());
}
```

Let's look at the ```impl``` block, everything within the ```impl``` block will be associated with the ```Rectangle``` type. In main, when we call the ```get_area``` function, we use the *method syntax*. Another thing to notice is that when implementing the ```get_area``` function, ```&self``` is a shorthand for:

```rust
impl Rectangle{
    fn get_area(self: &Self){
        self.width * self.height
    }
}
```

And the ```self``` is pointing to the reference that the instance that the function is defined under. In our case, we borrowed the reference immutably, method can also borrow the reference immutably or take ownership of ```Self```. If we want to take the reference and change the instance, we'd like to use:

```rust
impl Rectangle{
    fn change_width(&mut self){
        self.width = 50;
    }
}
```

Although directly taking the ownership in methods is rare, this often related to when you need to transform the instance into something else, and prevent the user to use the original instance later.

### Associated Functions

All functions that are defined under the ```impl``` block is considered as associated functions, the only difference is whether the function need the ```self``` parameter. If the function does not have the ```self``` parameter, it is considered as a static method, and we should use ```::``` syntax to access and invoke it. Like ```String::from()```. It is often used as constructors, but we do not need ```new``` syntax like in other languages:

```rust
impl Rectangle{
    fn sqr(size: u32)->Self{
        Self{
            width: size,
            height: size
        }
    }
}

fn main(){
    let sq = Rectangle::sqr(3);
}
```
