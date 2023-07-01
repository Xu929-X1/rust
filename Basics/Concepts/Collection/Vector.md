# Vector

Vector, with the syntax ```Vec<T>```, is a data structure allows you to store same type of data in a sequence. The data are stored next to each other in memory, they are useful when we need to store a list of items.

To create a new empty vector, we call the static method ```new```:

```rust
let v: Vector<i32> = Vector::new();
```

Note that since we are creating an empty vector, we need to specify the data type that it hold, otherwise rustc has no clue to infer what the type the vector should hold. If we creating a new vector with an initial value, rustc is able to infer the type, as it is provided. In addition, Rust provides a macro that allow you to create a new vector in a more convenient way:

```rust
let v = vec![1, 3, 5];
```

Note that since the initial value is provided as integers, the vector now has the type ```i32``` as it is the default integer type.

Now we have a vector in hand, we are going to look at how to read, and write to the data structure.

## Write

In the Rust book, it only introduces the method ```push```, but I'll cover more that I think would be useful in the daily development.

Before we start, I think it is important to introduce two different integer types: ```isize``` and ```usize```. As their name suggests, ```usize``` cannot be negative, and ```isize``` can. So ```usize``` is useful for addressing the size, memory address, and index, whereas ```isize``` is useful for addressing the offset for size, memory address and index.

### Add

- append

The method ```append``` takes two parameters, the first is the a mutable reference of a vector, and the second parameter is another mutable reference of another vector that you want to append to the first vector. What the method does is moving all the elements in the second vector to the first one. Note that the ```vector``` type has a data type associated with each of the instance, so we need to make sure the first and the second vector instance has the same data type that they hold. The method ```append``` is a great tool if we want to concatenate two vectors.

```rust
let mut v1 = vec![1,3];
let mut v2 = vec![2,4];

v1.append(&mut v2); // note the first parameter is &self and it is omitted, and for the second parameter, we need to specify the mutable reference

assert_eq!(v1, [1, 2, 3, 4]);
assert_eq!(v2, []);

```

- push

The method ```push``` takes two parameters, the first is a mutable reference of a vector instance, and the second is the value you wish to append to the vector instance. Note that the data type of the first and the second should align. For example, if you want to push a ```String``` to an ```i32``` vector, that is not going to work.

```rust
let mut v1 = vec![1, 2];

v1.push(3);

assert_eq!(v1, [1, 2, 3]);
```

Actually, for the methods that are implemented for ```vector```, checking the official document might be more helpful. And I'll save some time.

## Iteration

To access each element of a vector in turn, we would use the loop to iterate all the element rather than use indices to access element individually. If we just want to read the value, we could iterate through the immutable reference of the vector, and if we want to write or modify the elements in the vector, we could iterate through the mutable reference, note to dereference the element before apply any manipulations. Here are two examples:

```rust
//read through:
let v = vec![1, 2, 3];
for i in &v{
    println!("{i}");
}

//modify
let mut v2 = vec![1, 2, 3];
for i in &mut v2{//note here we use mutable reference
    *i *= 2; //deref first
}
```

## Use Enum to Store Multiple Types

Vector allows us store values with the same type in a consecutive block of memory, what if we need to store a list of items with different types? Well, we could utilize the enum type. We could use our enum type that is defined to represent element of different types, here is an example:

```rust
enum VectorElement{
    Int(i32),
    Float(f64),
    Text(String)
}

let v = vec![VectorElement::Int(1), VectorElement::Float(1.2), VectorElement::Text(String::from("Hello!"))];

```

But this technique only works when you know the exhaustive list of data types when you are programming. If the vector is dynamically constructed, we need to use a trait object.
