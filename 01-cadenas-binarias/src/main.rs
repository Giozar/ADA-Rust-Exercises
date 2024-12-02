use std::io;

fn main() {
    let mut arr: Vec<u32> = Vec::new();

    println!("Ingresa un dato");
    let mut data: String = String::new();
    io::stdin()
        .read_line(&mut data)
        .ok()
        .expect("No se pudo leer el dato");

    println!("El dato ingresado es {}", data);

    let tam: u32 = data.trim().parse().expect("Esto no es un número");

    for _ in 0..tam {
        arr.push(0);
    }

    println!("El arreglo inicial es: {:?}", arr);

    let modified_arr = add_one(arr, tam);

    println!("El arreglo modificado es: {:?}", modified_arr);
}

fn add_one(mut arr: Vec<u32>, tam: u32) -> Vec<u32> {
    let mut n: u32 = tam;

    if n > 0 {
        arr[n as usize - 1] = 1;
        n -= 1;

        println!("En proceso de pon 1 es: {:?}", arr);

        permutation(&mut arr, tam);

        return add_one(arr, n);
    }

    arr
}

fn permutation(arr: &mut Vec<u32>, tam: u32) -> &mut Vec<u32> {

    if tam <= 1 {
        return arr;
    }
    let mut m: usize = (tam - 1) as usize;
    
    let aux = arr[m];
    arr[m] = arr[m - 1];
    arr[m - 1] = aux;

    m -= 1;

    println!("En permutar es: {:?}", arr);

    return permutation(arr, m as u32);
}
