# Bringing path into scope with ```use``` keyword

In the [module](./module.md) and [path](./paths_and_pub.md) part, we introduced what a module is and how to find corresponding path to access desired module. But it seems redundant and verbose to specify the path every time we want to access them. We could create a shortcut to a path with ```use``` keyword once, and then use the shorter name to access the modules or items everywhere else in the scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

When we specify ```use crate::front_of_house::hosting``` in the current scope, we creating a valid name ```hosting``` in the current scope, and we can use the name ```hosting``` to access the module or item. When we use the ```use``` keyword, Rust will also check the privacy.

Note that Rust uses block scope, and the ```use``` only generate a valid shortcut in the current scope, so something like this will not compile:

```diff
- This will not compile
```

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); //there's no hosting shortcut in the customer scope
    }
}
```

---

## Providing new name using ```as``` keyword

In some cases, we are using two items from two modules that have the same name, for example, in our standard library, ```fmt``` and ```io``` both have a ```Result``` type, in this case, when bringing them into the scope, we can use ```as``` keyword to assign them a new name:

```rust
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn function1() -> FmtResult {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

```

---

## Re-exporting technique

We can also combine the ```pub``` and ```use``` keyword to make the shortcut public, this technique is called re-exporting. Remember we just talked about we need to bring the shortcut in the correct scope, this implies the shortcut is private for the scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Before, external code have to call ```restaurant::front_of_house::hosting::add_to_waitlist()``` to call, but with the current re-exporting, we sort of changed the structure, now the external code can also call ```restaurant::hosting::add_to_waitlist()```. This is useful when some details are hidden and you wish to restructure the path in a more reasonable way for your users.

---

## Using nested paths to clean up large use list

Sometimes we might use a lot of modules from a same library, to clean up the use list, we can do something like this:

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

//to

// --snip--
use std::{cmp::Ordering, io};
// --snip--

```

Looks somehow like destructuring.
