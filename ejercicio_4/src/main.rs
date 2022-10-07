use rand::Rng;

fn generar_datos() -> [[i32;2];25] {
    let mut arreglo : [[i32;2];25] = [[0;2];25];
    let mut num_a = rand::thread_rng();
    let mut num_b = rand::thread_rng();

    for num in 0..25 {
        arreglo[num][1] = num as i32 + 1;
    }

    loop {
        let presidente: i32 = num_a.gen_range(0..25);
        let votos: i32 = num_b.gen_range(-1..=1000000);

        if votos < 0 {
            return arreglo;
        }

        arreglo[presidente as usize][0] = votos;
    }
}


fn main() {
    let mut arreglo_principal: [[i32;2];25] = generar_datos();

    arreglo_principal. sort();

    println!("De menor a mayor cantidad de votos fue de la siguiente manera:\n");
    for index in 0..25 {
        if index < 24 {
            println!(">>> El presidente n°{} tiene {}", arreglo_principal[index][1], arreglo_principal[index][0])
        } else {
            println!("\n>>> El presidente con mas votos fue el n°{} y tiene {} votos\n", arreglo_principal[index][1], arreglo_principal[index][0])
        }
    }
}
