use std::io;

fn convert_to_int(s: &str) -> i32 {
    let x = s.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut entrada_fatorial = String::new();

    io::stdin()
        .read_line(&mut entrada_fatorial)
        .expect("Erro ao ler entrada_fatorial");

    let mut fatorial = 1;
    let mut entrada_int = convert_to_int(&entrada_fatorial);

    while entrada_int > 1 {
        fatorial *= entrada_int;
        entrada_int -= 1;
    }

    println!(
        "O fatorial de {} é {}",
        &entrada_fatorial.trim(),
        fatorial.to_string()
    );
}
