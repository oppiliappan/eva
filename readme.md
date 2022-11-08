
![heroimg.png](https://u.peppe.rs/6G.png)

# eva

simple calculator REPL, similar to `bc(1)`, with syntax highlighting and persistent history

![eva.png](https://u.peppe.rs/kP.png)

### installation

- Homebrew
```shell
$ brew install eva
```

- crates.io
```shell
$ cargo install eva
```

- manual
```shell
$ git clone https://github.com/nerdypepper/eva.git
$ cargo run
```

### usage

```shell
eva 0.3.1
NerdyPepper <akshayoppiliappan@gmail.com>
Calculator REPL similar to bc(1)

USAGE:
    eva [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Optional expression string to run eva in command mode

OPTIONS:
    -b, --base <RADIX>    Radix of calculation output (1 - 36) [default: 10]
    -f, --fix <FIX>       Number of decimal places in output (1 - 64) [default: 10]
    -h, --help            Print help information
    -r, --radian          Use radian mode
    -V, --version         Print version information

```

type out an expression and hit enter, repeat.

```shell
> 1 + sin(30)
1.5
> floor(sqrt(3^2 + 5^2))
5
> 5sin(45) + cos(0)
4.53553
```

### updating

 - crates.io
 ```shell
$ cargo install eva --force
 ```

 - manual
```shell
$ cargo install --force --path /path/to/eva
```

### operators

 - binary operators: `+ - * / ^ **`
 - unary operators: `+ -`

### constants

some constants available in rust standard library.

```
e      pi
```

examples:
```
pi * 5^2  # πr²
```

### functions

all trigonometric functions expect input in degrees.

```
1 argument:
sin    cos     tan    csc    sec    cot    sinh   cosh   tanh
asin   acos    atan   acsc   asec   acot   ln     log10  sqrt
ceil   floor   abs

2 arguments:
log    nroot

deg(x) - convert x to degrees
rad(x) - convert x to radians
```

examples:
```
sqrt(sin(30)) # parentheses are mandatory for functions

log10100      # no
log10(100)    # yes

log(1, 10)    # function with two arguments
```

### quality of life features

 - auto insertion of `*` operator
```
>12sin(45(2))             # 12 * sin(45 * (2))
12
```

 - auto balancing of parentheses
```
>ceil(sqrt(3^2 + 5^2      # ceil(sqrt(3^2 + 5^2))
6
```

 - use previous answer with `_`
```
> sin(pi)
0.0548036650
> _^2
0.0030034417
>
```

- super neat error handling
```
> 1 + ln(-1)
Domain Error: Out of bounds!
```

 - syntax highlighting

### todo

 - ~~add support for variables (pi, e, _ (previous answer))~~
 - ~~syntax highlighting~~
 - ~~multiple arg functions~~
 - ~~screenshots~~
 - ~~create logo~~
 - ~~unary operators (minus, plus)~~
 - ~~add detailed error handler~~
 - ~~add unit tests~~
 - ~~lineditor~~ with syntax highlighting
 - ~~add more functions~~

### contributors

the rust community has helped eva come a long way, but these devs deserve a
special mention for their contributions:

[Ivan Tham](https://github.com/pickfire)  
[Milan Marković](https://github.com/hepek)  
[asapokl](https://github.com/kzoper)  
