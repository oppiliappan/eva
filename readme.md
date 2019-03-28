
![heroimg.png](https://files.nerdypepper.me/6G.png)

# eva

simple calculator REPL, similar to `bc(1)`

![eva.png](https://files.nerdypepper.me/Kt.png)

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
$ eva
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

### operators

 - binary operators: `+ - * / ^`
 - unary operators: `+ -`

### functions

All trignometric functions expect input in degrees.

```
sin
cos 
tan 
csc  
sec 
cot 
sinh
cosh
tanh
ln 
log
sqrt
ceil
floor
deg(x) - convert x to degrees
rad(x) - convert x to radians
abs(x) - (x * x) ^ 0.5
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
12sin(45(2))  # 12 * sin(45 * (2))
ans: 12
```

 - auto balancing of parentheses
```
ceil(sqrt(3^2 + 5^2   # ceil(sqrt(3^2 + 5^2))
ans: 6
```

- super neat error handling
```
> 1 + ln(-1)
Domain Error: Out of bounds!
```

### todo

 - add support for variables (ans, pi, e)
 - multiple arg functions
 - screenshots
 - create logo
 - ~~unary operators (minus, plus)~~
 - ~~add detailed error handler~~
 - ~~add unit tests~~
 - ~~lineditor~~ with syntax highlighting
 - ~~add more functions~~
