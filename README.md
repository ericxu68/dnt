# dnt
simple date and time formatting using chrono crate


usage 
```rust
println!("{}", DnT::date(Local)); //Local is for device's local time
println!("{}", DnT::time(Local));
println!("{}", DnT::dateandtime(Local));
println!("{}", DnT::date(ZoneEast(5.5))); // for timezone towards east
println!("{}", DnT::time(ZoneEast(5.5)));
println!("{}", DnT::dateandtime(ZoneWest(5.5))); // for timezone towards west
```

