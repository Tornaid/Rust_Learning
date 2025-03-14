// 📖 Code Sheet - Sprint 2 : Ownership & Gestion de Mémoire 🚀
//
// 📌 Rust gère la mémoire avec un système d'ownership strict.
// 📌 Objectif : Éviter les erreurs mémoire et garantir la sécurité sans garbage collector.

// 🟢 1. Ownership (Propriété d'une variable)
// Chaque valeur en Rust a un "propriétaire" unique.

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;  // ❌ Ownership transféré → s1 n'est plus valide

    // println!("{}", s1); // 🛑 ERREUR ! s1 n'est plus accessible
    println!("{}", s2); // ✅ Fonctionne, s2 est le nouveau propriétaire
}

// Explication :
// - Quand `s1` est affecté à `s2`, la mémoire est déplacée, et `s1` devient invalide.
// - Pas de "double free" car Rust empêche `s1` d’être utilisé après le move.

// 🔵 2. Borrowing (`&` et `&mut`)
// Solution au problème de transfert de propriété : utiliser un emprunt (borrow).

fn afficher_longueur(s: &String) -> usize {
    s.len()  // ✅ Lecture autorisée
}

fn main() {
    let texte = String::from("Rust");
    let longueur = afficher_longueur(&texte);  // ⬅️ Passage par référence (borrow)
    println!("Longueur : {}", longueur);
}

// Explication :
// - `&String` passe une référence sans transférer l’ownership.
// - Cela permet de **lire sans modifier** la variable.

// 🟣 3. Emprunt mutable (`&mut`)
// On peut modifier une variable via un emprunt mutable.

fn ajouter_point(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut message = String::from("Hello");
    ajouter_point(&mut message);  // 🔄 Emprunt mutable
    println!("{}", message);  // ✅ "Hello!"
}

// Explication :
// - `&mut` permet de modifier la variable sans la déplacer.
// - ⚠️ **On ne peut pas avoir plusieurs emprunts mutables en même temps**.

// 🟠 4. Lifetimes (`'a`)
// Permet d’éviter les références invalides.

fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let string1 = String::from("Rust");
    let string2 = String::from("Langage");
    let resultat = plus_long(&string1, &string2);
    println!("Le plus long est : {}", resultat);
}

// Explication :
// - `'a` signifie que **la durée de vie des références est identique**.
// - Empêche d’avoir des références invalides.

// 📌 Objectif : Lire et tester ces codes pour bien comprendre la gestion mémoire en Rust.
// 🚀 Prochaine étape : Commencer les exercices pratiques !
