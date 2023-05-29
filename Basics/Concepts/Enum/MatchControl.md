# Match Control Flow Construct

Let's look at an example first:

```rust
enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_coin(coin: CoinType)->u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter=> 25,
    }
}
```

This give you a ```switch``` ```case``` impression, isn't it? But they are different, ```switch, case``` can only take number or string to match, but remember, a ```enum``` variant can hold any type of data. In a more precise description, a ```match``` compare against patterns, a pattern could be made up with literal values, variable names, wildcards and more. Using ```match``` control flow with the ```enum``` brings another benefit that we allow the compiler to check if every match pattern is considered.

---

## Patterns that binds to values

There are cases that an ```enum``` variant is bind with a value. For example, we are going to provide several known host address for our IoT devices, we could choose to do:

```rust
enum KnownAddressV4 {
    PC("127.0.0.1"),
    Remote("192.158.0.1"),

}
enum IPType {
    V4(KnownAddressV4),
    V6,
}

fn match_address(ip: IPType)->String{
    match ip {
        IPType::V6=>String::from("Some IPV6 Address"),
        IPType::V4(address)=>{
            println!("The Address is {:?}", address);
            address
        }
    }
}
```

---

## Match with Option\<T\>

Remember that ```Option<T>``` is also an ```enum``` type. Let us look at an example:

```rust
fn plus_one(x: Option<i32>)->Option<i32>{
    match x {
        Some(value) => Some(Value + 1),
        None => None,
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

---

## Catch all patterns and _ placeholder

There are other cases that you wish only to handle some ```enum``` variants with special operations, and the rest you have a general purpose handler for that, in this case, we could use ```other```. For example, we are processing some digits from a sensor, and we are looking specifically for the number 28 and 100. If the number is one of those two, we do something, if not, we do nothing but print that everything is normal. We could write:

```rust
let reading: u8 = 10;//or some reading from somewhere
match reading{
    28=>do_something(),
    100=>do_something_else(),
    other=>println!("Everything is normal, reading is {}", other),
}

fn do_something(){}
fn do_something_else(){}    
```

Think of ```other``` as a placeholder for any possible value for ```u8``` except for 28 and 100. If we do not care about the value at all, we can use ```_``` to catch all the rest possible cases.

```rust
fn monitor(reading: u8){
    28=>do_something(),
    100=>do_something_else(),
    _=>println!("Everything is normal, don't mind what the actual reading is"),
}
fn do_something(){}
fn do_something_else(){}    
```

---

## Concise control with ```if let```

To be frankly, the exhaustive requirement for ```match``` will help us avoid errors before runtime, but it is a bit too wordy, isn't it? Sometimes we just want a way to match few patterns and ignore the other patterns, this is when the ```if let``` come to play.

Let us look at an example, when we are using ```Option<T>```, most of the time we just want to do something while the pattern is ```Some```, and ignore the ```None```.

```rust
let sample = Some(0u8);
//using match
match sample {
    Some(i)=>println!("The sample is {}", i),
    _=>(),
}

//using if let
if let Some(i) = sample {
    println!("The sample is {}", i);
}else{
    //process data that does not match the pattern
}

```
