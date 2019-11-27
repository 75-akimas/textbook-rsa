fn main() {
    let p = 383;
    let q = 83;
    let e = 8963;
    let n = p.clone()*q.clone();
    println!("{} {}", n, fi(p, q));

    println!("public keys n: {} e: {}", n, e);
    let d = modinv(e, fi(p, q));
    println!("secret keys d: {} euler fi: {}", d, fi(p, q));
    let mut m = 150;
    let mut c = pow_mod(m, e, n);
    println!("plain: {} cipher: {}", m, c);
    println!("decipher : {}", pow_mod(c, d, n));
}

fn fi(p: i128, q: i128) -> i128 {
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
    let (mut x0, mut y0, mut x1, mut y1, mut a, mut b, mut q)= (1, 0, 0, 1, a, b, a/b);

    while b != 0 {
        q = a / b;
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
