use rand::Rng;

fn aleatorio_para_5(mut arreglo: [i32;5]) -> [i32;5] {
    let mut num = rand::thread_rng();
    for index in 0..=4 {
        let num_ingresar: i32 = num.gen_range(0..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}

fn aleatorio_para_6(mut arreglo: [i32;6]) -> [i32;6] {
    let mut num = rand::thread_rng();
    for index in 0..=5 {
        let num_ingresar: i32 = num.gen_range(0..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}

fn ordenar(mut arreglo_1: [i32;5], mut arreglo_2: [i32;6]) -> [i32;11] {
    // momento ordenar
    let mut arreglo_general : [i32;11] = [0;11];
    let mut contador: i32 = 0;

    loop {
        let mut num: i32 = 0;
        let mut aux: usize = 0;
        for index in 0..=10 {
            if index < 5 && arreglo_1[index] != -1 {
                if num < arreglo_1[index] {
                    num = arreglo_1[index];
                    aux = index;
                }
            } else if index > 4 && arreglo_2[index - 5] != -1 {
                if num < arreglo_2[index - 5] {
                    num = arreglo_2[index - 5];
                    aux = index;
                }
            }
        }

        if aux < 5 {
            arreglo_general[contador as usize] = num;
            arreglo_1[aux] = -1;
        } else {
            arreglo_general[contador as usize] = num;
            arreglo_2[aux - 5] = -1;
        }

        if contador == 10 {
            break;
        }

        contador += 1;
    }
    

    return arreglo_general;
}

fn main() {
    let mut arreglo_1 : [i32;5] = [0;5];
    let mut arreglo_2 : [i32;6] = [0;6];

    arreglo_1 = aleatorio_para_5(arreglo_1);
    arreglo_2 = aleatorio_para_6(arreglo_2);

    println!("{:?}", arreglo_1);
    println!("{:?}", arreglo_2);

    let arreglo_general: [i32;11] = ordenar(arreglo_1, arreglo_2);

    println!("{:?}", arreglo_general);
}
