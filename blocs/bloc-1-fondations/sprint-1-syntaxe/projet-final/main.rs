extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    println!("Bienvenue dans le jeu du Nombre Mystère !");
    let mut rng = rand::thread_rng(); // Utilisation correcte de thread_rng()
    let secret = rng.gen_range(1..=100); // Syntaxe confirmée fonctionnelle

    loop {
        println!("Entrez votre supposition :");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Erreur de lecture");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        if guess < secret {
            println!("Trop bas !");
        } else if guess > secret {
            println!("Trop haut !");
        } else {
            println!("Bravo, vous avez trouvé le nombre !");
            break;
        }
    }
}
