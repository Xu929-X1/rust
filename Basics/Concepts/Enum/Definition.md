# Define an Enum

This is pretty straight forward, so let us look at an example on how to define an ```enum``` for IP address type (IPV4 & IPV6):

```rust
enum IPType{
    v4,
    v6
}
```

Base on the ```enum``` we created instance of the two variants that ```IPType``` holds:

```rust
let four = IPType::v4;
let six = IPType::v6;
```

Then we can define a function that takes ```IPType```:

```rust
fn route(ip_kind: IPType){
    // some code
}

route(IPType::v4);
route(IPType::v6);
```

```enum``` cannot store values, but we can combine ```struct``` with ```enum``` to make more meaningful ```struct```, for example, we could to have a ```struct``` to represent different host address and the type of the address:

```rust
enum IPType {
    V4,
    V6
}

struct Host {
    kind: IPType,
    address: String,
}

let localhost = Host {
    kind: IPType::V4,
    address: String::from("127.0.0.1")
};

let remote = Host {
    kind: IPType::V6,
    address: String::from("::1")
};
```

However, representing the concept would be more concise with just ```enum``` than within ```struct```, we can associate a value with the ```enum``` variants like this:

```rust
enum IPType {
    V4(String),
    V6(String)
}

let localhost = IPType::V4(String::from("127.0.0.1"));
```

In this case, the name of the variants in the ```enum``` also becomes a function that constructs the instance of the ```enum```, that is, ```IPType::V4()``` is a function that takes a string and return an instance of ```IPType```. There is another advantage of using ```enum```, for example, we want to store IPV4 address in 4 ```u8``` digits, and ```string``` for IPV6. We can easily do this in ```enum```, and it would not be possible if we use ```struct```:

```rust
enum IPType {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum IPTypeInStruct{
    V4, 
    V6
}

struct IPAddress{
    type: IPTypeInStruct,
    address: String | (u8, u8, u8, u8)//????
}
```

Let us look at another example to see how ```enum``` and ```struct``` has similar aspects:

```rust
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

//the enum is similar to ...
struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct
```

If we are using ```enum```, we could just define a function that takes ```Message``` type as the parameter type. But if we are using ```struct```, we have to define different ```impl``` for each of the type:

```rust
//impl for enum 
impl Message {
    fn call(&self){
        //method body
    }
}

let m = Message::Write(String::from("Message"));
m.call();

//otherwise use struct, we have to define function for each
impl MoveMessage{
    fn call(&self){
        //method body
    }
}

let move_message = MoveMessage {
    x: 10,
    y: 20
}
move_message.call();

//... 2 more
```

Obviously, in this case, using ```enum``` is a more elegant choice.

---

## The Option Enum and Its Advantage Over Null Values

```Option``` is an ```enum``` type that is defined by the standard library. The definition of ```Option``` in the standard library is as follows:

```rust
enum Option<T>{
    None,
    Some(T),
}
```

Because ```Option``` type is so useful, we do not need to explicitly include it into the scope. Also, we can use ```Some``` and ```None``` directly without ```Option::``` prefix. Here are some examples to show how to use ```Some``` and ```None```:

```rust
let some_number = Some(5);
let some_char = Some('a');

let absent_number: Option<i32> = None;
```

Note that for ```absent_number```, we need to explicitly assign a type ```Option<i32>``` to the variable, because rustc cannot infer the type from just ```None```.

The reason for having a ```Option``` type is that Rust does not have a null value.
Many times there are chances we could use null value as non-null one, cause ```NullPointerException```. In my practice, we use Typescript in our web development, we need to write defensive code, or assert the type for a nullable value. But not everyone would pay attention to this at all time, so Rust has this particular ```Option``` implementation. To force the compiler to check if we are trying to use a nullable value. This is achieved by forcing the user to convert ```Option<T>``` to ```T``` before using the value or performing operations. So ```Option``` is a good way to represent data if we know that the data is nullable. There are a lot of methods that are defined for the ```Option``` type, such as ```is_some()``` to determine if a ```Option``` instance holds some value, and ```unwrap_or()```, which will take a ```Option<T>``` instance if it has ```T``` type value, and takes a second parameter which will handle if the instance is ```None```.
