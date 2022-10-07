use rand::Rng;
use std::io::stdin;

fn aleatorio(mut arreglo: [i32;12]) -> [i32;12] {
    let mut num = rand::thread_rng();
    for index in 0..=11 {
        let num_ingresar: i32 = num.gen_range(1..=200);
        arreglo[index] = num_ingresar;
    }
    return arreglo;
}


fn ordenar(mut arreglo: [i32;12]) -> [i32;12] {
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

    for index in 0..=11 {
        if index != 0 && index != 11 {
            arreglo_aux[index] = -arreglo_aux[index];
        }
    }
    
    return arreglo_aux;
}


fn numero_negativo(arreglo: [i32;12], mut numero_negativo: i32) -> i32 {
    for index in 1..=10 {
        if arreglo[index] < 0 {
            numero_negativo += 1;
        }
    }
    return numero_negativo;
}


fn mostrar(arreglo: [i32;12], _cambios: bool, _momento: bool, vidas_perdidas: i32 ) {
    if _momento == false {
        println!("-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-");
        println!(">>>>> Ha perdido en total {} vidas", vidas_perdidas)
    } else {    
        if _cambios == true {
            println!(">>>>> Se ha desbloqueado algun numero");
        } else {
            println!(">>>>> Ha perdido una vida")
        }
    }

    println!("\n>>> TABLERO <<<");

    for index in 0..=11 {
        if index == 0 {
            print!("")
        }
        if arreglo[index] > 0 {
            print!("{} ", arreglo[index]);
        } else if arreglo[index] < -99 {
            print!("*** ")
        } else if arreglo[index] < -9 {
            print!("** ")
        } else {
            print!("* ")
        }
    }

    println!("");
}


fn main() {
    let mut arreglo_principal : [i32;12] = [0;12];
    let mut vidas_perdidas: i32 = 0;
    let mut _cambios: bool = false;
    let mut _momento: bool = false;

    arreglo_principal = aleatorio(arreglo_principal);

    arreglo_principal = ordenar(arreglo_principal);
    println!("");

    loop {
        _cambios = false;
        _momento = false;
        let mut numeros_n_1: i32 = 0;
        let mut numeros_n_2: i32 = 0;

        mostrar(arreglo_principal, _cambios, _momento, vidas_perdidas);
        println!("\n>>>>>Ingrese el numero debajo");
        let mut numero_texto: String = String::new();
        stdin().read_line(&mut numero_texto).unwrap();
        let numero: i32 = numero_texto.trim().parse().unwrap();
        println!("");

        numeros_n_1 = numero_negativo(arreglo_principal, numeros_n_1);
        
        if numero < 0 || numeros_n_1 == 0{
            panic!("El juego termino, perdio {} vidas", vidas_perdidas);
        }
        
        for index in 1..=10 {
            if numero == -arreglo_principal[index] {
                arreglo_principal[index] = -arreglo_principal[index]
            }
        }

        numeros_n_2 = numero_negativo(arreglo_principal, numeros_n_2);

        if numeros_n_1 == numeros_n_2 {
            vidas_perdidas += 1;
        } else {
            _cambios = true;
        }

        _momento = true;
        mostrar(arreglo_principal, _cambios, _momento, vidas_perdidas);
        println!("");
    }
}
