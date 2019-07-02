
![heroimg.png](https://files.nerdypepper.me/6G.png)

# eva

simple calculator REPL, similar to `bc(1)`, with syntax highlighting and persistent history

![eva.png](https://files.nerdypepper.me/kP.png)

### installation

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
eva 0.2.4
NerdyPepper <akshayoppiliappan@gmail.com>
Calculator REPL similar to bc(1)

USAGE:
    eva [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -h, --help       Prints help information
    -r, --radian     set eva to radian mode
    -V, --version    Prints version information

OPTIONS:
    -b, --base <RADIX>    set the radix of calculation output (2, 8, 10, 16 etc.)
    -f, --fix <FIX>       set number of decimal places in the output

ARGS:
    <INPUT>    optional expression string to run eva in command mode

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

 - binary operators: `+ - * / ^`
 - unary operators: `+ -`

### functions

all trignometric functions expect input in degrees.

```
sin    cos     tan    csc    sec    cot    sinh   cosh   tanh
asin   acos    atan   acsc   asec   acot   ln     log    sqrt
ceil   floor   abs

deg(x) - convert x to degrees
rad(x) - convert x to radians
```

examples:  
```
sqrt(sin(30)) # parentheses are mandatory for functions

log100        # no
log(100)      # yes
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

- super neat error handling
```
> 1 + ln(-1)
Domain Error: Out of bounds!
```

 - syntax highlighting

### todo

 - add support for variables (ans, pi, e)
 - ~~syntax highlighting~~
 - multiple arg functions
 - ~~screenshots~~
 - ~~create logo~~
 - ~~unary operators (minus, plus)~~
 - ~~add detailed error handler~~
 - ~~add unit tests~~
 - ~~lineditor~~ with syntax highlighting
 - ~~add more functions~~
