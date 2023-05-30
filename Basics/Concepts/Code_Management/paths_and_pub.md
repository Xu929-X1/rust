# Paths that referring to an item in the module tree

Like the directory system, the module system in Rust also have ```path``` that allow users to navigate to the desired items. There are two types of ```path```: relative path and absolute path, note for the absolute path, it starts with the literal ```crate```. And relative path can use ```super```, ```self``` or an identifier in the current module.

The module in Rust is private by default, which means it cannot be accessed by other module or parent module. If we wish to expose a specific module, we need to use ```pub``` keyword.

The ```super``` keyword is like ```..``` in filesystem. If we want to construct a path start with parent module, we can start with ```super```. For example, a function inside a module wants to access a function that is defined in the parent module:

```rust
fn deliver_order() {}

mod back_of_house {
    
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //here use super to goto parent module scope
    }

    fn cook_order() {}
}
```

---

## Privacy for Struct and Enums

When we want to make a ```struct``` public, notice that we need to not only use the ```pub``` keyword before the ```struct``` definition, we also need to use the ```pub``` keyword for the fields in the ```struct``` that we want to make public

Filename:  *src/lib.rs*

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In the case we are working with ```enum```, once the ```enum``` is public, all of its variants are public.
