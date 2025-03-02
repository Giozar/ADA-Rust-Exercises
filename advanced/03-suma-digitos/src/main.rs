use std::io;
fn count_digit_strings(s: u32) -> u32 {
    fn count_rec(s: i32) -> u32 {
        if s == 0 {
            return 1;
        }
        if s < 0 {
            return 0;
        }

        count_rec(s - 1) +
        count_rec(s - 2) +
        count_rec(s - 3) +
        count_rec(s - 4) +
        count_rec(s - 5) +
        count_rec(s - 6) +
        count_rec(s - 7) +
        count_rec(s - 8) +
        count_rec(s - 9)
    }

    count_rec(s as i32)
}


fn main() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).ok().expect("No se pudo leer el dato");

    let n: u32 = data.trim().parse().expect("No es un número");
    println!("{}", count_digit_strings(n));
}
