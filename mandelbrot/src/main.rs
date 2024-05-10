use num::Complex;

fn main() {
    complex_square_add_loop(Complex { re: 0.24, im: 0.3 });
}

/*
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..1000 {
        z = z * z + c;
    }
    println!("z = {:?}", z);
}
*/

/*
    Tenta determinar se 'c' esta no conjunto de Mandelbrot,
    usando no maximo 'limit' iteracoes para decidir.
    Se 'c' nao for um membro, retorna 'Some(i)', onde 'i' e o numero de
    iteracoes que 'c' levou para escapar do circulo de raio 2.
    Se 'c' parece ser um membro (mais precisamente, se atingimos o limite
    de iteracoes sem que 'c' escape), retorna 'None'.
*/
fn escape_time(c: Complex<f64>, limit: u32) -> Option<uzise> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
