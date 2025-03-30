# 3.1

Rust variables are immutable by default. `mut` makes them mutable.

Constants are declared using `const` and must be evaluated at compile time.

Can shadow variables by using `let $NAME` multiple times. Can redefine `$NAME`'s type this way too. `mut` doesn't allow for changing types like that.

```rust
// OK
let spaces = "   ";
let spaces = spaces.len();
// NOT OK
let mut spaces = "    ";
spaces = spaces.len();
```

`let` is just allowing us to assign a new variable to the same name, whereas `mut` allows us to assign a new value to the same variable, which keeps its name.

# 3.2

The compiler is pretty smart when it comes to types. Check the guessing game project out and change the type of the parsed guess--the compiler will automatically infer that the stored guess is of the same type. This isn't just promotion--the compiler is assigning the same type to the guess and the secret since it knows we compare them later, and one has unspecified type. 

There are some nice ways to handle overflow:

> * Wrap in all modes with the wrapping_* methods, such as wrapping_add.
> * Return the None value if there is overflow with the checked_* methods.
> * Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
> * Saturate at the value’s minimum or maximum values with the saturating_* methods.

`char` is 4bytes long and is a single Unicode value (Unicode Scalar Value). This means `let heart_eyed_cat = '😻';` is permissible.

Tuples are fixed length, mixed type lists. Access arguments through pattern matching

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {y}");
```

Alternatively use `.$INDEX` to access at specific indices

```rust
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

`()`, unit, is of course default return value.

Arrays are fixed length, homogenous type lists.

Vectors are dynamic length, but the book puts off discussing them till later.

Annotations include type and length.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Constant arrays are defined somewhat similarly:

```rust
let a = [3; 5];
```

Indexing is as expected, using `array[$INDEX]` syntax.

Attempting to access an array at an invalid index at runtime will cause a panic, which causes the program to quit safely--this disallows unsafe memory access.

# 3.3

Use snake_case.

Must declare parameters' types in function signature

Statements vs expression

`let` is a statement. Statements don't return values

Curlies introduce a new scope.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

`x` holds no value outside of the let binding.

Expressions don't end with semicolons--putting a `;` changes an expression into a statement, and so won't return a value.

Just like in R, last value returned implicitly. Unlike R, you need to declare return type of functions.

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Statements "evaluate" to unit, and so that can introduce errors:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

# 3.4

Use `//` for comments. Multi line comments need marker on each line.

Doc comments will be addressed later.

# 3.5

Can use `if condition VALUE1 else VALUE2` in let bindings. Both values must be of the same type though. The following won't compile:

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

Can use the `loop` construct on RHS of `let` bindings too. You can `break` with a value too.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 20
}
```
Label loops using `'NAME` syntax.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

Standard for-each loop.

Construct ranges using `(#..#)` syntax:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```