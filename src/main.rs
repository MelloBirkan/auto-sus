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
    match nota as i8 {
        92..=100 => {
            println!("Análise do resultado: Melhor imaginavel");
        }
        85..=91 => {
            println!("Análise do resultado: Excelente");
        }
        72..=84 => {
            println!("Análise do resultado: Bom");
        }
        52..=71 => {
            println!("Análise do resultado: Ok/Justo");
        }
        38..=51 => {
            println!("Análise do resultado: Ruim");
        }
        0..=37 => {
            println!("Análise do resultado: Pior imaginavel");
        }
        _ => panic!("Análise do resultado: Nota invalida")
    }
}

fn main() {
    let respostas: [i8; 10] = [5, 1, 4, 2, 5, 1, 3, 2, 5, 3];
    classificacao_sus(math_sus(&respostas));
}

