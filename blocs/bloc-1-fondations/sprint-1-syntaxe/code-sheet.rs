// 📖 Code Sheet - Sprint 1 : Syntaxe Rust 🚀
//
// 📌 Cette code sheet explique en détail chaque notion abordée dans le Sprint 1.

// 🟢 1. Variables et Mutabilité
// Rust est un langage qui favorise la sécurité mémoire et la prévisibilité du code.
// Par défaut, les variables sont immuables, ce qui signifie que leur valeur ne peut pas être modifiée après l'assignation.

fn main() {
    let x = 5; // Variable immutable (on ne peut pas modifier sa valeur après l'assignation)
    println!("Valeur initiale de x : {}", x);

    // x = 10; // ❌ ERREUR : Tentative de modification d'une variable immutable

    let mut y = 10; // Variable mutable (on peut changer sa valeur)
    println!("Valeur initiale de y : {}", y);

    y = 20; // ✅ Autorisé car 'y' a été déclarée comme mutable avec 'mut'
    println!("Nouvelle valeur de y : {}", y);
}

// Explication :
// - `let x = 5;` crée une variable **immutable**.
// - `let mut y = 10;` crée une variable **mutable**, permettant de changer sa valeur après l'assignation.
// - En Rust, l'immutabilité empêche des erreurs imprévues et garantit que la valeur ne sera pas modifiée accidentellement.

// 🔵 2. Conditions et Boucles
// Les conditions permettent d'exécuter différentes branches de code en fonction d'une condition donnée.

fn main() {
    let nombre = 7;

    if nombre < 10 {
        println!("Petit nombre !");
    } else {
        println!("Grand nombre !");
    }

    // Boucle for : Permet de répéter une action un certain nombre de fois.
    for i in 1..=5 {  // '1..=5' signifie "de 1 à 5 inclus"
        println!("Compteur : {}", i);
    }

    // Boucle while : Continue tant que la condition est vraie.
    let mut compteur = 3;
    while compteur > 0 {
        println!("Décompte : {}", compteur);
        compteur -= 1;
    }
    println!("🚀 Lancement terminé !");
}

// Explication :
// - `if` fonctionne comme en C/C++, Python ou JavaScript.
// - `for i in 1..=5` parcourt les nombres de 1 à 5 inclus.
// - `while` répète l'action tant que la condition est vraie.

// 🟣 3. Fonctions et Gestion des Erreurs
// Les fonctions permettent d'organiser et de réutiliser du code.

fn addition(a: i32, b: i32) -> i32 {
    a + b // Pas besoin d'écrire 'return', la dernière expression est retournée automatiquement
}

// Rust utilise un système de gestion des erreurs puissant basé sur `Result<T, E>`

fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division par zéro interdite".to_string()) // Retourne une erreur sous forme de chaîne de caractères
    } else {
        Ok(a / b) // Retourne le résultat sous forme de 'Ok'
    }
}

fn main() {
    println!("Résultat de l'addition : {}", addition(10, 5));

    // Gestion des erreurs avec 'match'
    match division(10, 2) {
        Ok(res) => println!("Résultat division : {}", res),
        Err(e) => println!("Erreur : {}", e),
    }

    match division(10, 0) {
        Ok(res) => println!("Résultat division : {}", res),
        Err(e) => println!("Erreur : {}", e),
    }
}

// Explication :
// - `addition(a: i32, b: i32) -> i32` définit une fonction qui prend deux entiers et retourne un entier.
// - `Result<T, E>` est un type utilisé pour gérer les erreurs sans faire planter le programme.
// - `match` permet de traiter les cas `Ok` et `Err` proprement.

// 🟠 4. Manipulation des Entrées Utilisateur
// Rust permet de lire des entrées utilisateur via la bibliothèque standard 'std::io'

use std::io;

fn main() {
    println!("Entrez votre nom :");

    let mut nom = String::new(); // Création d'une chaîne mutable pour stocker l'entrée utilisateur
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");

    println!("Bonjour, {} !", nom.trim()); // 'trim()' enlève les espaces et sauts de ligne inutiles
}

// Explication :
// - `io::stdin().read_line(&mut nom)` permet de lire une ligne entrée par l'utilisateur et de la stocker dans `nom`.
// - `expect("Erreur de lecture")` affiche un message d'erreur si l'entrée échoue.
// - `nom.trim()` enlève les espaces inutiles à la fin de la saisie.

// 📌 Objectif : Lire et tester ces codes pour bien comprendre la syntaxe de Rust.
// 🚀 Prochaine étape : Commencer les exercices pratiques !
