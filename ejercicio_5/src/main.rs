use rand::Rng;

fn main() {
    let mut num_a = rand::thread_rng();
    let mut num_b = rand::thread_rng();
    let n: i32 = num_a.gen_range(1..25);
    let m: i32 = num_b.gen_range(1..25);
    let mut arreglo_principal: [[i32;25];25] = [[-1;25];25];
    let mut aux: i32 = 0;

    for index_n in 0..n {
        for index_m in 0..m{
            if n == 0 {
            arreglo_principal[index_n as usize][index_m as usize] = index_m;
            } else {
            arreglo_principal[index_n as usize][index_m as usize] = (index_m) + (index_n) + aux;
            }

            if index_m == m - 1 {
                aux += m - 1
            }
        }
    }

    println!("");
    for index_n in 0..n {
        for index_m in 0..m {
            if index_m == 0 {
                print!("> {} ", arreglo_principal[index_n as usize][index_m as usize]);
            } else if index_m == m - 1 {
                print!("{} ", arreglo_principal[index_n as usize][index_m as usize]);
                print!("\n");
            } else {
                print!("{} ", arreglo_principal[index_n as usize][index_m as usize])
            }
        }
    } 
    println!("");
}
