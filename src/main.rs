extern crate rand;
use rand::prelude::*;
fn main() {
//    let p = 383;
//    let q = 83;
//    let e = 8963;
    let p = gen_safe_prime() as i128;
    let q = gen_safe_prime() as i128;
    let e = gen_safe_prime() as i128;
    let n = p.clone()*q.clone();
    println!("{} {}", n, totient(p, q));

    println!("public keys n: {} e: {}", n, e);
    let d = modinv(e, totient(p, q));
    println!("secret keys d: {} euler totient: {} p: {} q: {}", d, totient(p, q), p, q);
    let m = 999992;
    let c = pow_mod(m, e, n);
    println!("plain: {} cipher: {}", m, c);
    println!("decipher : {}", pow_mod(c, d, n));
}

fn gen_safe_prime() -> u16 {
    let mut rng = thread_rng();
    let mut candidate = rng.gen();
    while !miller_rabin(candidate as i128) || !miller_rabin(((candidate-1)/2) as i128) {
        candidate = rng.gen();
    }
    return candidate;
}

fn miller_rabin(n: i128) -> bool {
    let k: usize = 1000;
    if n == 2 {
        return true;
    }
    if n == 1 || n & 1 == 0 {
        return false;
    }
    let mut d = n - 1;
    while d & 1 == 0 {
        d >>= 1;
    }

    let mut rng = thread_rng();
    for _ in 0..k {
        let a = rng.gen_range(1, n-1);
        let mut t = d;
        let mut y = pow_mod(a, t, n);
        while t != n-1 && y != 1 && y != n-1 {
            y = pow_mod(y, 2, n);
            t<<=1;
        }
        if y != n-1 && t & 1 == 0 {
            return false;
        }
    }
    return true;
}

fn totient(p: i128, q: i128) -> i128 {
    return (p-1) * (q-1)
}

fn pow_mod(x: i128, n: i128, m: i128) -> i128 {
    let mut x = x.clone();
    let mut i = 1;
    let mut p = 1;
    while i <= n {
        if (i & n) != 0  {
            p = p * x % m;
        }
        x = x * x % m;
        i <<= 1;
    }
    return p;
}

fn xgcd(a:i128, b:i128) -> (i128, i128, i128) {
    let (mut x0, mut y0, mut x1, mut y1)= (1, 0, 0, 1);
    let (mut a, mut b) = (a, b);

    while b != 0 {
        let q = a / b;
        let tmp = b;
        b = a % b;
        a = tmp;

        let tmp = x1;
        x1 = x0 - q * x1;
        x0 = tmp;
        let tmp = y1;
        y1 = y0 - q * y1;
        y0 = tmp;
    }
    return (a, x0, y0);
}

fn modinv(a: i128, m: i128) -> i128 {
    let (_g, x, _y) = xgcd(a, m);
    return (x + m) % m;
}
