# Gussing Game is just an Input Output tutorial of Rust

## Input Data form Console Lib
> Using `io` library defined in `std` library Equally `C Language` to input data

> Using 

```{RUST}

    io::stdin()
        .read_line(&mut guess)
        
```


# Immutable vs Mutable Declrations
`mut` => Mutable, this variable can be changed after creation

```{RUST}
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

## Associated Functionality
> The `::` syntax in the `::new` line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.\

## Handling Potential Failure with the Result Type
> Handling data exeptions using `.expect("Failed to read line");`
If you don't expect the build will warn you about the line that can has potintial failure like  `read_line` 