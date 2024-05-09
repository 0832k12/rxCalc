## How To Use
Place this piece of code at the top of your file:
```rust
use rxcalc::calc;
```
Then, you can use rcalc like this:
```rust
calc("1*(1+1)"); //return 2
```
You can also use it as a separate project:
```sh
$ cargo run -- '1*(1+1)'
2
```
[source code](https://github.com/rsc-project/rxcalc)