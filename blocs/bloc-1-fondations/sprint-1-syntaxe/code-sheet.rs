
// 📖 Code Sheet - Sprint 1 : Syntaxe Rust 🚀

// 📌 Résumé des notions abordées :

// 🟢 1. Variables et Mutabilité
fn main() {
    let x = 5; // Variable immutable
    let mut y = 10; // Variable mutable
    println!("x = {}, y = {}", x, y);
    y = 20;
    println!("Nouvelle valeur de y = {}", y);
}

// 🔵 2. Conditions et Boucles
fn main() {
    let nombre = 7;
    if nombre < 10 {
        println!("Petit nombre !");
    } else {
        println!("Grand nombre !");
    }

    for i in 1..=5 {
        println!("Compteur : {}", i);
    }
}

// 🟣 3. Fonctions et Gestion des Erreurs
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division par zéro interdite".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("Résultat : {}", addition(10, 5));

    match division(10, 0) {
        Ok(res) => println!("Résultat division : {}", res),
        Err(e) => println!("Erreur : {}", e),
    }
}

// 🟠 4. Manipulation des Entrées Utilisateur
use std::io;

fn main() {
    println!("Entrez votre nom :");
    
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    
    println!("Bonjour, {} !", nom.trim());
}

// 📌 Objectif : Lire et tester ces codes pour bien comprendre la syntaxe de Rust.
// 🚀 Prochaine étape : Commencer les exercices pratiques !
