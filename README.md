# `enum_for_matches`

`enum_for_matches` runs a match arm for each enum variant passed into it
regardless of type. So, you can make a string out of an enum which wraps
numeric types individually, such as `serde_value::Value` for example. See
README.md on GitHub for more information.

For example, this:

```rust
enum TestEnum {
    I64(i64),
    U64(u64)
}

let e = TestEnum::I64(80);
let mut s = String::new();

enum_for_matches::run!(e, {TestEnum::I64(i) | TestEnum::U64(i)}, {s = i.to_string();});
eprintln!("{}", &s);
```

Would expand to:

```rust
match e
{
    TestEnum::I64(i) => { s = i.to_string(); }
    TestEnum::U64(i) => { s = i.to_string(); } 
    _ => { }
}
```

And print `80`.

## Contributing

This crate is considered feature complete. It uses a copious amount of
`.clone()`'s, many of which are probably removable. However, since this only
runs at compile time, on probably quite small vectors, I couldn't be bothered.
