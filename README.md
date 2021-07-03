# tide-errors
If `preroll` errors are too prescriptive for you, but you don't want to write boilerplate to convert errors into JSON errors.

Convert `Result<T, E2>` into `Result<Message<T, E1>, E2>` where `Message<T, E1>: Into<Result<tide::Response, tide::Error>>`

Bring your own JSON Error type, or use the default error message.

## Example

Use the default JSON errors.


```rust
    convert!(db::create_some_object(...).await, ok => Message::created, DatabaseConstraintConversion::<(), UniqueConstraint>::default(), DatabaseConstraintConversion::<(), ForeignKeyConstraint>::default())
```

Use your own JSON error

```rust
    let options = MyOptions::new(...);

    convert!(db::create_some_object(...).await, ok => Message::created, DatabaseConstraintConversion::<_, UniqueConstraint>::with_options(options.clone()), DatabaseConstraintConversion::<_, ForeignKeyConstraint>::with_options(options))
```