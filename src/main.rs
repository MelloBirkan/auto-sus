fn math_sus(respostas: &[i8; 10]) -> f32 {

    let total: f32;
    let mut total_par = 0;
    let mut total_impar = 0;

    for i in 0..10 {
        if 0 == (i+1) % 2 {
            total_par += 5 - respostas[i];
        } else {
            total_impar += respostas[i] - 1;
        }
    }
    total = (total_par + total_impar) as f32 * 2.5;
    println!("A pontuação foi de: {total}");
    return total;
}

fn classificacao_sus(nota: f32) {

}

fn main() {
    let respostas: [i8; 10] = [5, 1, 4, 2, 5, 1, 3, 2, 5, 3];
    math_sus(&respostas);
}

