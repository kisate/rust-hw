fn max(array: &[i32; 10]) -> i32 {
    let mut ans = array[0];
    for x in array {
        if x > &ans {
            ans = *x
        }
    }
    ans
}

fn min(array: &[i32; 10]) -> i32 {
    let mut ans = array[0];
    for x in array {
        if x < &ans {
            ans = *x
        }
    }
    ans
}

fn is_prime(n: &i32) -> bool {
    let mut i = 2;
    while i as f32 <= (*n as f32).sqrt() {
        if n % i == 0 {
            return false;
        }
        i += 1
    }
    true
}

fn nth_prime(n: i32) -> i32 {
    let mut i = 0;
    let mut p = 2;
    loop {
        if is_prime(&p) {
            i += 1
        }
        if i == n {
            break p
        }
        p += 1;
    }
}

fn bin_search(array: &[i32; 10], x: &i32, l: usize, r: usize) -> usize {
    if l + 1 == r {
        return r;
    }
    let m = l + (r - l) / 2;
    if array[m] <= *x {
        return bin_search(array, x, m, r);
    }
    else {
        return bin_search(array, x, l, m);
    }
}

fn main() {
    let array = [1,2,3,4,5,6,7,8,9,10];
    println!("{}", max(&array));
    println!("{}", min(&array));
    println!("{}", nth_prime(10));
    println!("{}", bin_search(&array, &5, 0, 10));
}
