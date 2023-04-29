# Slice Type

This is a brand new topic for me. In short, a slice is a partial reference to a
value. Here is a [small practice file](../../SlicePractice/src/main.rs) that I
wrote regarding the slice type.

Imagine a business logic requires you to get the first n words from a string.
And we also know that the string does not include any symbol and it has at least
on word, so what we need to do is only get the space and use that to indicate
the end of a word. But the reference type will give us the whole thing, this is
where the slice type comes in handy.

First thing we need to do is to check every character in the string to see if it
is a space:

```rust
let s = String::from("Random words");

fn first_word(s: &String)->usize{

    //convert the string to byte array:
let bytes = s.as_bytes();
//use the iterator to iterate the byte array, enumerate method returns a tuple, the first is the index, the second is the reference of current element:
    for(i, &reference) in bytes.iter().enumerate(){
        if(&reference == b' '){
            return i;
        }
        //if no space, the whole string is considered as a word
        s.len()
    }
}
```

Now we get the index of where the space appears, we want to cut from the
beginning of the string to the index and use the slice as our result. But note
that the returned index is only valid for `&String`, if the string is changed
after the first_word function is invoked, there is no guarantee the index will
still point to the space.

# String Slice

In this case, we could use the string slice to get the slice of the string. Here
is an example of simple usage of string slice:

```rust
let my_str = String::from("Hello World");

let hello = &my_str[0..5];
let world = &my_str[6..11];
```

The string slice is a reference to a portion of a string, the brackets specifies
the start and end index, and are connected by the range syntax `..`. The start
index can be omitted if the index is 0, and the end index can be omitted if the
index is the last. To get the whole string:

```rust
let hello_world = &my_str[..];
```

Note the slice range indices must occur at valid UTF-8 character boundaries. (if
you don't know, utf-8 is a 1-4 byte encoding)

The type notation of string slice is `&str`, let us look back to the first word
problem, this time, we want to return the string slice instead of the index:

```rust
fn first_word(s: &String)->&str{
    let bytes = s.as_bytes();
    for(i, &element) in bytes.iter().enumerate(){
        if(&element == b' '){
            return &s[..i];
        }
    }
    &s[..]
}
```

Now with this, we have a cleaner function to extract the first word of a string, because the compiler will make sure the reference to the ```String``` to be valid, remember when we get the index and then clear the string, then try to access the character at the index, and get an error at runtime? Now we can spot the issue before the runtime with the help from compiler:

```diff
- The following code will not compile
```

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!
    /**
    Compiling ownership v0.1.0 (file:///projects/ownership)
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    --> src/main.rs:18:5
    |
    16 |     let word = first_word(&s);
    |                           -- immutable borrow occurs here
    17 |
    18 |     s.clear(); // error!
    |     ^^^^^^^^^ mutable borrow occurs here
    19 |
    20 |     println!("the first word is: {}", word);
    |                                       ---- immutable borrow later used here

    For more information about this error, try `rustc --explain E0502`.
    error: could not compile `ownership` due to previous error
    */

    println!("the first word is: {}", word);
}
```

Because if we only get the number, it is not associated with the string reference, so the compiler would not know we are going to use the string reference after using ```s.clear()```, but if we use the slice reference, the compiler would know after we borrowed the mutable reference, we are going to use immutable reference, and this violate the rules.

## String Literal as Slices

We know that the string literal being stored inside the binary.

```rust
let s = "Hello World!";
```

Here the type of ```s``` is ```&str```, it is a slice pointing to the specific point in the binary, this is why the string literals are immutable. ```&str``` is an immutable reference.

## Use String Slice as Parameters

We can improve the first word function by changing the type of the parameter from ```&String``` to ```&str```, as we can pass either a slice or the reference of the string. In this way it is more flexible.

*Note the ```&String``` reference is equivalent to the whole slice of the string*
