# ZenLang > Common mistakes

## Nested dot indexing an array

You would think you'd need something like this:

```
let x.1.0 = 0;
```

This does not work, because the tokenizer makes float 1.0 of the index

Instead, do this:

```
let x.1 .0 = 0;
```
