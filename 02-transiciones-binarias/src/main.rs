use std::io;
fn main() {

    // println!("Ingresa un dato");
    let mut data = String::new();
    io::stdin().read_line( &mut data).ok().expect("No se pudo leer el dato");
    // println!("El dato es {}", data.trim());

    let n: u32 = data.trim().parse().expect("No es un número");

    println!("{}", transiciones(n));

}


fn transiciones (n: u32) -> u32 {

    if n < 2 {
        return  0;
    }

    return  2 * transiciones(n - 1) + 2_u32.pow(n - 1);
    
}
