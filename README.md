# blocks

`blocks` is a simple left-to-right parser of balanced blocks.

Example:

```rust
use blocks::Blocks;

let code = "[[]]";
let mut blocks = Blocks::new();

for (n, c) in code.chars().enumerate() {
    match c {
        '[' => blocks.add_left(n),
        ']' => blocks.add_right(n)?,
        _ => unreachable!()
    }
}

let blocks = blocks.consume()?;

for block in blocks {
    println!("{:?}", block);
}
```
