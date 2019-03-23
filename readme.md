# eva

a read-eval-print-loop, similar to `bc(1)`

### installation

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
1 + sin(30)
ans: 1.5

floor(sqrt(3^2 + 5^2))
ans: 5

5sin(45) + cos(0  # eva will try to autobalance braces
ans: 3.4999999999999996
```

### operators

 - binary operators: `+ - * / ^`
 - unary operators: `+ -`

### functions

fn(x: Number) -> Number

```
sin(x) |    
cos(x) | 
tan(x) | 
csc(x) | -> x is in degrees  
sec(x) | 
cot(x) | 
sinh(x)
cosh(x)
tanh(x)
ln(x)     # log to the base e
log(x)    # log to the base 10
sqrt(x)   # x ^ 0.5
ceil(x)
floor(x)
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

### todo

 - add detailed error handler
 - multiple arg functions
 - ~~unary operators (minus, plus)~~
 - screenshots
 - lineditor with syntax highlighting
 - ~~add more functions~~
