# Bit Manipulation

> let `n` be a number

> let `k` be index of bit in that number

### Set Bit 
Changing kth bit to 1
```
n = n | (1 << k)
```

### Unset Bit 
Changing kth bit to 0
```
n = n & !(1 << k)
```

### Toggle Bit 
Toggling kth bit
```
n = n ^ (1 << k)
```

### Printing Binary
```
for i in (0..=31).rev() {
    print!("{}", (n >> i) & 1);
}
```
