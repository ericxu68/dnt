# dnt
simple date and time formatting using chrono crate


usage 
```rust
println!("{}", DnT::date(Local)); //Local is for device's local time
println!("{}", DnT::time(Local));
println!("{}", DnT::dateandtime(Local));
println!("{}", DnT::date(Timezone(5.5))); // time zone is for time-zone + towards east currently
println!("{}", DnT::time(Timezone(5.5)));
println!("{}", DnT::dateandtime(Timezone(5.5)));
```

