extern crate rand;
use rand::prelude::*;
fn main() {
    let p = gen_safe_prime() as i128;
    let q = gen_safe_prime() as i128;
    let e = gen_safe_prime() as i128;
    let n = (p as i128)*(q as i128);
    println!("{} {}", n, euler_totient(p, q));

    println!("public keys (n: {} e: {})", n, e);
    let d = modinv(e, euler_totient(p, q));
    println!("secret keys (d: {} euler_totient: {} p: {} q: {})", d, euler_totient(p, q), p, q);

    let plain = "Hello, world.";
    let cipher = plain.as_bytes().iter().map(|&x| pow_mod(x as i128, e, n)).collect::<Vec<i128>>();
    println!("plain: {} cipher: {:?}", plain, cipher.as_slice());
    let deciper = cipher.iter().map(|&x| pow_mod(x, d, n) as u8 as char).collect::<String>();
    println!("decipher : {:?}", deciper);
}

fn gen_safe_prime() -> i128 {
    let mut rng = thread_rng();
    let mut candidate: u16 = rng.gen();
    while !miller_rabin(candidate as i128) || !miller_rabin(((candidate-1)/2) as i128) {
        candidate = rng.gen();
    }
    return candidate as i128;
}

fn miller_rabin(n: i128) -> bool {
    let k: usize = 20;
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

fn euler_totient(p: i128, q: i128) -> i128 {
    return (p-1) * (q-1)
}

fn pow_mod(x: i128, n: i128, m: i128) -> i128 {
    let mut x = x as i128;
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
