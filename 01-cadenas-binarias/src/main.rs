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

    if tam == 0 {
        return  arr;
    }

    arr[tam as usize - 1] = 1;
   
    println!("En proceso de pon 1 es: {:?}", arr);

    permutation(arr.clone(), tam);

    return add_one(arr, tam -1 );
}

fn permutation(mut arr: Vec<u32>, tam: u32) -> Vec<u32> {
    if tam <= 1 {
        return arr;
    }

    let m: usize = (tam - 1) as usize;

    if m > 0 {
        let aux = arr[m];
        arr[m] = arr[m - 1];
        arr[m - 1] = aux;
        println!("En permutar es: {:?}", arr);
    }

    permutation(arr, tam - 1)
}
