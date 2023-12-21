fn top_down(n: usize, a: &mut [u128]) -> u128 {
    if a[n] != 0 || n == 0 {
        return a[n];
    }
    a[n] = top_down(n-1, a) + top_down(n-2, a);
    return a[n];
}

fn bottom_up(n: usize, a: &mut [u128]) {
    for i in 2usize..=n {
        a[i] = a[i-1] + a[i-2];
    }
}

fn fibonacci(n: usize, a: &mut [u128]) {
    a[0] = 0;
    a[1] = 1;

    // for bottom up approach
    bottom_up(n, a);
    // this approach is little bit faster cuz iteration is used

    // for top down approach
    // a[n] = top_down(n, a);
}

fn main() {
    const N: usize = 186;
    // 186th fibonacci is the max fibonacci value that a u128 can store
    let mut a: [u128; N+1] = [0; N+1];
    fibonacci(N, &mut a);
    for i in 0usize..=N {
        println!("{} -> {} ", i, a[i]);
    }
}
