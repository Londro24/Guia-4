use std::io::stdin;
use rand::Rng;

fn panico(nombre_empresa: String) {
    let aux: &str = nombre_empresa.trim();
    if aux == "" || aux == " " || aux == "  " {
        panic!("Ese no es un nombre valido")
    } 
}

fn generar_produccion(mut produccion_arroz: [i32;12]) -> [i32;12] {
    for x in 0..=11 {
        let mut tonelada = rand::thread_rng();
        let tonelada_en_mes: i32 = tonelada.gen_range(80..=200);
        produccion_arroz[x] = tonelada_en_mes;
    }
    return produccion_arroz;
}

fn generar_promedio(produccion_arroz: [i32;12]) -> i32 {
    let mut suma: i32 = 0;
    for x in produccion_arroz.iter() {
        suma += x;
    }
    let promedio: i32 = suma / 12;
    return promedio;
}

fn mes(x: usize) -> &'static str {
    let texto = match x {
        0 => "enero",
        1 => "febero",
        2 => "marzo",
        3 => "abril",
        4 => "mayo",
        5 => "junio",
        6 => "julio",
        7 => "agosto",
        8 => "septiembre",
        9 => "octubre",
        10 => "noviembre",
        11 => "diciembre",
        _ => ""
    };
    return &texto;
}

fn generar_meses(mut mejores_meses : [&str;12], produccion_arroz: [i32;12], promedio_anual: i32) -> [&str;12] {
    for x in 0..=11 {
        if produccion_arroz[x] > promedio_anual {
            mejores_meses[x] = mes(x)
        }
    }
    return mejores_meses;
}

fn generar_numero_magico(produccion_arroz: [i32;12]) -> i32 {
    let mut numero: i32 = 0;
    let mut mejor_arroz: i32 = 0;
    for x in 0..=11 {
        if produccion_arroz[x] > mejor_arroz {
            mejor_arroz = produccion_arroz[x];
            numero = x as i32;
        }
    }
    return numero;
}

fn main() {
    println!("Bienvenido, este es un programa que genera aleatoriamente una 
base de datos respecto al arroz producido en un año de enero a diciembre. 
Presenta el promedio anual, los mejores meses y el mes de mayor producción.");
    println!("Por favor ingrese un nombre de su empresa ficticia");
    let mut nombre_empresa: String = String::new();
    stdin().read_line(&mut nombre_empresa).unwrap();
    let nombre_aux: String = nombre_empresa.trim().to_string();

    panico(nombre_empresa);

    let mut produccion_arroz: [i32;12] = [0;12];
    produccion_arroz = generar_produccion(produccion_arroz);

    let promedio_anual: i32 = generar_promedio(produccion_arroz);

    let mut mejores_meses: [&str;12] = ["-1";12];
    mejores_meses = generar_meses(mejores_meses, produccion_arroz, promedio_anual);

    let numero_magico: i32 = generar_numero_magico(produccion_arroz);
    
    println!("");
    println!(">> El promedio anual de la empresa {} en su producción de arroz fue {} toneladas", nombre_aux, promedio_anual);
    print!(">> Los meses donde la producción de arroz sobre paso al promedio fueron ");
    let mut contador: u8 = 0;
    for x in 0..=11 {
        if mejores_meses[x] != "-1" {
            print!("{}, ", mejores_meses[x]);
            contador += 1;
        }
    }
    print!("dejando un total de {} meses sobre el promedio.", contador);
    println!("");
    println!(">> El mejor mes fue {} con una producción de {} toneladas de arroz.", 
    mejores_meses[numero_magico as usize], produccion_arroz[numero_magico as usize]);



}
