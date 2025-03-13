// Implémente une fonction qui divise deux nombres et renvoie une gestion d'erreur si le dénominateur est zéro.

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Erreur : Division par zéro !".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match division(10, 2) {
        Ok(result) => println!("Résultat de la division : {}", result),
        Err(e) => println!("{}", e),
    }

    match division(10, 0) {
        Ok(result) => println!("Résultat de la division : {}", result),
        Err(e) => println!("{}", e),
    }
}
