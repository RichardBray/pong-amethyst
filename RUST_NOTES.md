# Rust specific notes

I started to write this in the readme but it was getting a bit long so I've moved it to here.

Here are all the notes I've made on Rust whilst learning it. They exist for reference reasons only but maybe they'll help someone others learn too.


## Imports

https://doc.rust-lang.org/std/keyword.use.html

external:
```rs
use path::to::item;
use my_library::some_function();
```

internal/module:
```rs
mod include_me;
```


### Ownership system

Cannot use original variable when it has been assigned to something else, when it's ownership has been moved.
```rs
let v = vec![1, 2, 3];

let v2 = v;

println!("v[0] is: {}", v[0]);
// error: use of moved value: `v`
// println!("v[0] is: {}", v[0]);
```
Reason being memory for the variable in Rust is allocated on the Stack (memory address of the heap allocation to internal pointer) and Heap (for the actual data). These two parts of the varialbe must agree with each other at all times.

When a variable is reassigned rust does a bitwize copy ot the stack allocation which means there will be two interal pointers pointing to the same heap allocation. This violates Rust's safety by introducing a data race. One pointer could modify the heap allocation before another one does.

### Copy
Primitive types don't have this issue because they implement the copy trait which means accessing the original binding when owenership is stransfered will still work because their ownership is moved and this works because there is no pointer to heap data.


## Borrowing system
Allow the ownership of a variable binding to be borrowed by another variable making the original binding still accessible. Rust uses something called references to achieve this.

reference = reference and existing variable, don't occupy memory/storage, java and haxe automatically make references without specific syntax for it https://www.cs.utah.edu/~germain/PPS/Topics/pointers_and_references.html

immutable reference:
```rs
let x = 10;
let xr = &x;

println!("x is {}", xr);
```

mutable reference:
```rs
let mut x = 10;
let xr = &mut x;

*xr += 1;

println!("x is {}", xr);
```

### Rules
- any borrow must last for the scope
- only alowed one mutable reference in the scope
- or multiple immutable references not both

these rules prevent data races


## Lifetimes system

Since references are only valid for the scope they are in when a reference is used an an argument in a function, (struct or impl) it's sometimes safe to say what scope they are in so Rust knows where to find them.

```rs
fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
}

let line = "lang:en=Hello World!"; // scope a
let lang = "en";

let v;
{
    let p = format!("lang:{}=", lang); // scope b
    v = skip_prefix(line, p.as_str());
} 
println!("{}", v); // returns scope a
// Without lifetimes this wouldn't work
```

