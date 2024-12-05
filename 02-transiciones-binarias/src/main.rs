use std::io;

fn main() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).ok().expect("No se pudo leer el dato");

    let n: u128 = data.trim().parse().expect("No es un número");

    println!("{}", transiciones(n));
}

fn transiciones(n: u128) -> u128 {
    if n < 2 {
        return 0;
    }
    2 * transiciones(n - 1) + pow_u128(2, n - 1)
}

fn pow_u128(base: u128, exp: u128) -> u128 {
    base.pow(exp as u32)
}
