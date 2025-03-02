use std::io;

fn main() {
    
    println!("Ingresa un dato");
    let mut data: String = String::new();
    io::stdin()
        .read_line(&mut data)
        .ok()
        .expect("No se pudo leer el dato");

    println!("El dato ingresado es {}", data);

    let tam: usize = data.trim().parse().expect("Esto no es un número");

    let mut arr: Vec<i32> = vec![0; tam];

    permutation(&mut arr, tam);

}

fn permutation(arr: &mut Vec<i32>, tam: usize) {
    if tam == 0 {
        println!("{:?}", arr);
        return;
    }

    arr[tam - 1] = 1;
    permutation(arr, tam - 1);

    arr[tam - 1] = 0;
    permutation(arr, tam - 1);
}
