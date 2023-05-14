# Reference

To start with this chapter, let us look at an example, if I want get a string's
length and do some operations after, with the ownership rules in mind, my code
would look like this:

```rust
fn main(){
    let s1 = String::from("Init");
    let (s2, len) = get_length(s1);
    println!("The string {} has length of {}", s2, len);
}

fn get_length(string: String){
    let len = string.len();
    (string, len)//return a tuple, note in Rust, elements in an array must be the same type
}
```

This is counterintuitive, right? Normally I just want the `get_length` function
to return a number, instead of a tuple that returns the ownership of the value.
Ideally, I hope there is a way that can avoid the returning ownership process,
so my code is more readable for those developers who might not familiar with
Rust. In this case, I want to use `reference`.

To use the reference to an object, we simply put a `&` mark before the variable
that owns the value to indicate that we are using a reference. In type
declaration, the prefix `&` is also needed as it is indicating a reference type:

```rust
fn main(){
    let s1 = String::from("Init");
    let len = get_length(&s1); //note here no ownership transfer happended
    println!("The string {} has length of {}", s1, len);
}

fn get_length(string: &String){
    let len = string.len();
    len //return only the length
}
```

Looks much better! Paired with the reference operator `&`, we also have a
dereference operator `*`, this will be introduced later.

## Mutable References

Now, I want to modify the value of `s1` holds, maybe add something? here is what
I thought

```diff
- The following code will not compile
```

```rust
fn main(){
    let s = String::from("Init");

    update(&s);
    println!(s);//I'd expect to see the stdout: Init finished, now processing
}

fn update(s: &String){
    s.push_str(" finished, now processing");
}
```

You'll get an error says
``cannot borrow `*s` as mutable, as it is behind a `&` reference``. The same as
the variables, the reference is immutable by default. So let us change the code
so it would work:

```rust
fn main(){
    let mut s = String::from("Init"); // this has to be mutable at first place

    update(&mut s);
    println!(s);//I'd expect to see the stdout: Init finished, now processing
}

fn update(s: &mut String){
    s.push_str(" finished, now processing");
}
```

And there is a restriction on using mutable references: if you already have a
mutable reference to a value, you cannot have more reference to the same value.
I think this is to prevent race condition, think about this:

```diff
- The following code will not compile
```

```rust
fn main(){
    let mut s = String::from("Good");

    thread_one(s); //this might take longer, but who knows 
    thread_two(s); //this might take longer, but who knows 

    println!(s);
}

fn thread_one(s: &mut String){
    //we do some stuff here, like mutate the s
}
fn thread_two(s: &mut String){
    //we do some stuff here, like mutate the s
}
```

In this fashion, ```thread_one``` and ```thread_two``` are running in parallel, but there's no guarentee on how long they will take. So eventually, the value of ```s``` could be either the result of ```thread_one``` or ```thread_two```.

This, instead of race condition, has another name ```data race```, it will happen in the following conditions:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

We could use the block scope to avoid the data race:

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Also, the mutable reference cannot combine with the usage of immutable reference:

```diff
- The following code will not compile
```

```rust
let mut s = String::from("whatever");

let s_ref_one = &s;
let s_ref_two = &s;//no problem here
let s_ref_mut_one = &mut s;//this will panic
```

This is because the user of the immutable reference would not like the value to change under their nose. Just like React's state, one reference can only have one version at one time. Reference's scope starts with the time that it is introduced and ended where it is lastly used, so we could have a mutable reference after the immutable reference has been used:

```rust
let mut s = String::from("whatever");

let s_ref_one = &s;
let s_ref_two = &s;//no problem here

println!("{} and {} are used!", s_ref_one, s_ref_two);
let s_ref_mut_one = &mut s;//this is ok
```

A quick summary of what we just learned:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
