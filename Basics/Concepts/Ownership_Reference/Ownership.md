**From this part, the main function declaration will be omitted, just for me can
focus more on the content instead of programming patterns**

# Ownership

Ownership is a new concept in rust, but before we dive in, it is important to
review heap and stack, and how they are used for storing values.

## Heap & Stack

In computer memory, there are two types of memory heap and stack for different
purposes. Stack is an organized structure usually managed by the OS, which means
as a programmer, we do not explicitly allocate or drop data in the stack. Heap,
on the other hand, is a set less structured data, the location in memory for
heap is stored in a linked list, so accessing the designated memory is slower
for a heap. In some language programmers can allocate or drop the data stored in
heap at their will.

## Ownership Rules

- Each value in rust has a owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped

In Rust book, it uses `string` type to explain the ownership and borrowing, so
we are also going to use that, as it is easier.

There are many ways to use a string variable. For example, we can use string
literals:

```rust
let s1 = "Hello";
```

In this case, the variables is not mutable, this is because the size of the
variable is fixed when the string literal is assigned to it, and the compiler
will not take this literal into neither stack or heap, but just hardcode it into
the executable file. But normally, in our development, the most cases that I
encountered is either the string will be modified, or it is dynamically
generated. In those cases, the size of the string cannot be determined when we
are writing our code, so that in compiling phase, the compiler needs to assign a
size changable block of memory for the variable, the block of memory is located
in heap.

To do so, Rust provides a string type, `String`, the data is stored in the heap
so that it is able to store the text that the size is unknown during the compile
time. To create a `String` from a literal, the `String` type provides a
function:

```rust
let s1 = "Hello";
let s2 = String::from(s1);
```

Also, the `s2` can be mutable:

```rust
let s1 = "Hello";
let mut s2 = String::from(s1);
s2.push_str(", world!");

println!("{}", s2);//Hello, World!
```

Another thing is, rust has block scope, so similar to JavaScript, the variable
is valid within the scope of its definition, and will be dropped outside of the
scope. Because of this behavior, we need to look at another case that might
cause security vulnerabilities:

```rust
{
    let s1 = "Hello";
    let s2 = String::from(s1);
    let s3 = s2;
}
//when s2 and s3 gets out of the scope, they will both be dropped, but they are pointing to the same address in the heap, so that memory block will be dropped twice
```

To address this issue, let us come back to the second ownership rule:
`There can be only one owner at a time`. So in Rust, when assigning s2 to s3, we
actually needs to copy the heap data of s2, not the memory reference that is
stored in the stack, Rust provides a `clone` method to do so:

```rust
{
    let s1 = "Hello";
    let s2 = String::from(s1);
    s3 = s2.clone();
}
```

## Copy Trait and Stack data

With the ownership that we just discussed, let us look at this example:

```rust
let n1 = 1;
let n2 = n1;
```

In this example, we did not invoke the `clone` method, and this snippet is still
valid. That is because the size of `i32` is known through the runtime, so we
store it in the stack. In this case, there is no difference between deep and
shallow copy. In fact, Rust has a `Copy` trait for the values that stores in the
stack, if a type has `Copy` trait implemented, variables that uses it will not
base on the ownership rules, but simply copies the value.

The types that has `Copy` trait implementation:

- `All the integer types, such as u32.The Boolean type, bool, with values true
  and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example,
  (i32, i32) implements Copy, but (i32, String) does not.

## Function and Ownership

Another major difference that Rust has with other lanuguages that I am familiar
with, such as JavaScript, is when passing a parameter into a function, the
ownership will transfer, let us look at an example:

```rust
fn main(){
    let s = String::from("String!"); // s owns the "String!" 
    take_ownership(s); //the ownership has transfered into the function
    //s is no longer valid here
}

fn take_ownership(some_string: String){
    println!("The string {}'s ownership is now transfered to the parameter");
}
```

If we wish to use the variable `s` after the function execution, we can return
the value so that we could use it after the function:

```rust
fn main(){
    let s = String::from("String!"); // s owns the "String!" 
    let s2 = take_ownership(s); //the ownership has transfered into the function
    //s is no longer valid here
    //but we can use s2 here
    println!("The value {} has been returned to vairable s2!", s2);

}

fn take_ownership(some_string: String){
    println!("The string {}'s ownership is now transfered to the parameter");
    some_string //this returns the some_string
}
```

TBH this is a strange pattern, and it would cause a lot of mind burdens when the complexity of the code grows, and you need to keep track of the ownership status. But we have a feature that allow us to use the value without tranferring the ownership, which is called ```reference```.