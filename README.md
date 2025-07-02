# Construct Into

This macro creates a `construct` method, but all the parameters of `impl Into<T>` for the corresponding field.

```rust
#[derive(ConstructInto)]
pub struct S {
    a: A,
    b: B,
    c: C
}
// generates
impl S {
    pub fn construct(a: impl Into<A>, b: impl Into<B>, c: impl Into<C>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}
```

This ideal for when a struct has many complex types and/or new-type structs.

## Example

Given

```rust
pub struct Possibility(bool);
pub struct CustomVec<T>(Vec<T>);

#[derive(ConstructInto)]
pub struct Object {
    name: String,
    perhaps: Possibility
    list: CustomVec<T>,
}
```

I can write

```rust
let object = Object::construct(
    "this is my name",
    true,
    vec![1, 2, 3]
);
```

Instead of

```rust
let object = Object::new(
    "this is my name".into(),
    true.into(),
    vec![1, 2, 3].into(),
);
```

or

```rust
let object = Object {
    name: "this is my name".into(),
    perhaps: true.into(),
    list: vec![1, 2, 3].into(),
);
```
