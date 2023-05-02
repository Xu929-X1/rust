# Structure

Structure, or ```struct``` is like attributes of an object.

## Definition and Initiation

To define a ```struct```, we enter the keyword ```struct``` and then follow it the structure of it, for example:

```rust
struct Student  {
    name: String,
    age: i8,
    class: String,
    graduated: bool
}
```

Then, after defining a ```struct```, to create an instance of it, we can use the following structure, which is, the name of the structure, and followed by the specific key-value pair:

```rust
let bobby = Student {
    name: String::from("Bobby"),
    age:18,
    class: String::from("Chem"),
    graduated: false
}
```

To access the specific attribute, we use dot notation. If the ```struct``` is mutable, we can change the value of the attributes. Rust does not allow only mark the certain fields as mutable, so only the whole structure can be mutable.

```rust
let mut student1 = Student {
    name: String::from("Bobby"),
    age: 18,
    class: String::from("Physics"),
    graduated: false
};

//change a field
student1.graduated = true;
```

## Creating Instance from Other Instance with Struct Update Syntax

Imagine we are going to create another profile for a student with a different name, the age, class, and graduation status is the same as ```student1```, we can code the info in again, but for convenience, we can use ```..``` syntax:

```rust
let mut student2 = Student {
    name: String::from("John"),
    ..student1
};
```

Note that the ```..``` syntax must come last to specify that other field other than ```name``` is the same as in ```student1```, it is a bit different than JavaScript's spread syntax, which come first and then overwrite fields after the spread syntax.

Another important thing is, after we declared and assigned value to ```student2``` like we did, we cannot use ```student1``` after the creation of ```student2```, this is because the String in ```class``` is moved from ```student1``` to ```student2```. ```student2``` must contain a new class field to make ```student1``` remain valid. (Remember, ```String``` type does not have ```Copy``` trait implementation)

## Using Tuple Structs Without Named Fields

This brings me back to GLSL, in GLSL, there are a lot of pre-defined variables, like ```gl_FragCoord``` or ```gl_FragColor```. They are usually ```vec3``` or ```vec4``` type, in rust, similar to this, we can give a tuple a name and structure it into a meaningful variable, for example, if we want to use rgb and location to describe a pixel in a 3D space, we could use:

```rust
struct Color(i32, i32, i32);
struct Pos(f64, f64, f64);

fn main(){
    let black = Color(0, 0, 0);
    let origin = Pos(0., 0., 0.);
    //access values: 
    let r = black.0;
    let x = origin.0;
}
```

Similar to this:

```c
vec4 col = vec4(1., 1., 1., 1.);
vec3 pos = vec3(0., 0., 0.);
```

## Unit-like Struct Without Any Fields

Unit type is useful when we want to implement trait but do not want to store any value in the type itself. We will dive deeper into it later, now let us only look at the syntax:

```rust
struct AlwaysEqual;

fn main(){
    //instantiate
    let eq = AlwaysEqual;
}
```

There is another part we did not cover, which is use the slice type in the struct. Before we dive into that, we need to look at how we use ```String``` in the previous example and understand deeper meaning of it. Using ```String``` type means we want the instance of the ```struct``` to own all of its data and for that data, it should be valid as long as the ```struct``` is valid.

It is also possible to have ```struct``` own reference or slices that owned by something else, but to do so, we'll need ```lifetimes```. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. We will talk about it later.
