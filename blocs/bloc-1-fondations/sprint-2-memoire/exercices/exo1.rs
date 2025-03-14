// 📝 Exercice 1 : Comprendre le transfert d’ownership (Move)

fn afficher_message(msg: String) {
    println!("{}", msg);
    // Ici, `msg` est détruit à la fin de la fonction, ce qui entraîne la perte de son ownership.
}

fn main() {
    let message = String::from("Salut, Rust !");
    afficher_message(message); // 🔄 Ownership transféré ici

    // println!("{}", message); // ❌ ERREUR : 'message' n'est plus valide

    // Explication :
    // - La variable `message` contient un String alloué sur le tas.
    // - Lorsqu'on le passe à `afficher_message`, la propriété est transférée (move).
    // - À la fin de `afficher_message`, la mémoire est libérée.
    // - Rust empêche donc d'accéder à `message` après l’appel.