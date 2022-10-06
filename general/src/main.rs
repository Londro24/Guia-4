use rand::Rng;

fn aleatorio(mut arreglo: [i32;12]) -> [i32;12] {
    let mut num = rand::thread_rng();
    for index in 0..=11 {
        let num_ingresar: i32 = num.gen_range(0..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}


fn ordenar(mut arreglo: [i32;12]) -> [i32;12] {
    // momento ordenar
    let mut arreglo_aux : [i32;12] = [0;12];
    let mut contador: i32 = 0;

    loop {
        let mut num: i32 = 0;
        let mut aux: usize = 0;
        for index in 0..=11 {
            if arreglo[index] != -1 {
                if num < arreglo[index] {
                    num = arreglo[index];
                    aux = index;
                }
            }
        }

        arreglo_aux[contador as usize] = num;
        arreglo[aux] = -1;

        if contador == 11 {
            break;
        }

        contador += 1;
    }
    
    return arreglo_aux;
}

/*
fn censura(mut arreglo: [&str;12]) -> [&str;12] {
    let mut arreglo_aux : [&str;12] = ["-1";12];
    let mut contador: i32 = 0;


    loop {
        if contador != 0 && contador != 11 {

        }

        if contador == 11 {
            break;
        }

        contador += 1;
    }

    return ["-1";12];
}
 */


fn main() {
    let mut arreglo_principal : [i32;12] = [0;12];

    arreglo_principal = aleatorio(arreglo_principal);
    println!("{:?}", arreglo_principal);

    arreglo_principal = ordenar(arreglo_principal);
    println!("{:?}", arreglo_principal);

    let x: usize = 1;

    for index in 0..=11 {
        if index != 0 && index != 11 && index != x{
            print!("** ")
        } else {
            print!("{} ", arreglo_principal[index])
        }
    }

}
