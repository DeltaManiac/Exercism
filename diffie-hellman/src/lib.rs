use num_bigint::ToBigInt;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let p = p.to_bigint().unwrap();
    let g = g.to_bigint().unwrap();
    let a = a.to_bigint().unwrap();
    g.modpow(&a, &p).to_str_radix(10).parse::<u64>().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let p = p.to_bigint().unwrap();
    let b_pub = b_pub.to_bigint().unwrap();
    let a = a.to_bigint().unwrap();
    b_pub
        .modpow(&a, &p)
        .to_str_radix(10)
        .parse::<u64>()
        .unwrap()
}
