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

all trignometric functions expect input in degrees.  
conversion to radians is to be done manually (` x * 3.14 / 180`)

```
sin  
cos  
tan  
csc     # cosec is for pesants
sec  
cot  
sinh 
cosh 
tanh 
ln      # log to the base e
log     # log to the base 10
sqrt    # x ^ 0.5
ceil 
floor
```

examples:  
```
sqrt(sin(30)) # parenstheses are mandatory for functions

log100        # no
log(100)      # yes
```

### quality of life features

auto insertion of `*` operator
```
12sin(90)  # 12 * sin(90)
ans: 12

(5 + 6)(6 + 7)  # (5 + 6) * (6 + 7)
ans: 143

11(12)  # 11 * (12)
ans: 132
```

auto balancing of parentheses
```
ceil(sqrt(3^2 + 5^2
ans: 6
```

### todo

 - add detailed error handler
 - multiple arg functions
 - ~~unary operators (minus, factorial)~~
 - lineditor with syntax highlighting
 - ~~add more functions~~
