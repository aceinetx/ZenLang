# ZenLang > defer

Defers allow you to execute code at the end of a block:

```rust
fn main {
    {
        let x = 0;
        defer let x = 5;
        let x = 1;
    }
    return x; // returns 5
}
```

The example above uses a one-line defer, there is also a block defer:

```rust
fn main {
    {
        let x = 0;
        defer {
            let x = 5;
        }
        let x = 1;
    }
    return x; // returns 5
}
```

The behavior of defer is kinda _interesting_. You may see, in the above example we have a useless block in a function, the thing is, defers run code at the end of the block, passing return. That means, without this block, defers will place code after the return, which will not be reached
