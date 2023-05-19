use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn math_sus(respostas: &[i8; 10]) -> f32 {
    let total: f32;
    let mut total_par = 0;
    let mut total_impar = 0;


    for i in 0..10 {
        if 0 == (i + 1) % 2 {
            total_par += 5 - respostas[i];
        } else {
            total_impar += respostas[i] - 1;
        }
    }
    total = (total_par + total_impar) as f32 * 2.5;
    println!("A pontuação foi de: {}", total);
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
        _ => panic!("Análise do resultado: Nota invalida"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut nota_final: f32 = 0.0;
    let mut count: i8 = 0;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open("src/teste-usa.csv")?);

    for result in rdr.records() {
        let record = result?;

        // Começa a partir do índice 1 para ignorar o campo "Carimbo de data/hora"
        let mut respostas: [i8; 10] = [0; 10];
        for (i, field) in record.iter().enumerate().skip(1).take(10) {
            respostas[i-1] = field.parse::<i8>().unwrap_or(0);
        }

        count += 1;
        let individual = math_sus(&respostas);
        classificacao_sus(individual);
        nota_final += individual/count as f32;
    }

    Ok(())

}
