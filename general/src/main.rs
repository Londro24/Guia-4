use rand::Rng;

fn aleatorio_para_5(mut arreglo: [i32;5]) -> [i32;5] {
    let mut num = rand::thread_rng();
    for index in 0..=5 {
        let num_ingresar: i32 = num.gen_range(0..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}

fn aleatorio_para_6(mut arreglo: [i32;6]) -> [i32;6] {
    let mut num = rand::thread_rng();
    for index in 0..=6 {
        let num_ingresar: i32 = num.gen_range(0..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}

fn ordenar(arreglo_1: [i32;5], arreglo_2: [i32;6]) -> [i32;11] {
    // momento ordenar
    return [0;11];
}

fn main() {
    let mut arreglo_1 : [i32;5] = [0;5];
    let mut arreglo_2 : [i32;6] = [0;6];

    arreglo_1 = aleatorio_para_5(arreglo_1);
    arreglo_2 = aleatorio_para_6(arreglo_2);

    let mut arreglo_general : [i32;11] = [0;11];

    arreglo_general = ordenar(arreglo_1, arreglo_2);

}
