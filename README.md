```rust
mod stdlib;

fn main {
    println("Welcome to the ZenLang github repo!");
    return null;
}
```
### Roadmap
- [x] Error handling
- [x] `if let` and `elif let`
- [x] File read/write functions
- [x] Function attributes like #[naked]
- [x] vmcall keyword
- [x] mod and dynmod keyword
- [x] module loading from files (depends on above)
- [ ] more stdlib functions
  - [ ] ord
  - [ ] chr
  - [ ] number (convert str to number)
  - [ ] stringify (convert any to string)
  - [ ] boolean (convert str to boolean)
- [ ] debug info
- [ ] (not planned) cli debugger
