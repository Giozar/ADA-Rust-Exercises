use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    // Obtenemos el tamaño del arreglo y lo convertimos a f64 para el cálculo
    let n = arr.len() as f64;
    
    // Variables para contar positivos, negativos y ceros
    let mut count_positive = 0;
    let mut count_negative = 0;
    let mut count_zero = 0;
    
    // Recorrer el arreglo y contar cada tipo de número
    for &num in arr {
        if num > 0 {
            count_positive += 1;
        } else if num < 0 {
            count_negative += 1;
        } else {
            count_zero += 1;
        }
    }
    
    // Imprimir los ratios con 6 decimales cada uno
    println!("{:.6}", count_positive as f64 / n);
    println!("{:.6}", count_negative as f64 / n);
    println!("{:.6}", count_zero as f64 / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Leer el primer número que indica el tamaño del arreglo (aunque no se usa directamente en la lógica)
    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Leer la línea con los números, separarlos y convertirlos a i32
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
