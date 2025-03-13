// Demande à l’utilisateur un nombre et affiche s’il est pair ou impair.

use std::io;

fn main() {
    println!("Entrez un nombre :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");

    let nombre: i32 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    if nombre % 2 == 0 {
        println!("{} est un nombre pair.", nombre);
    } else {
        println!("{} est un nombre impair.", nombre);
    }
}
