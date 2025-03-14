
// 📝 Exercice 2 : Borrowing (Emprunt)

fn afficher_message(msg: &String) { // 📌 Emprunt non-mutables (lecture seule)
    println!("{}", msg);
}

fn longueur(msg: &String) -> usize { // 📌 Permet de lire sans prendre possession
    msg.len()
}

fn main() {
    let message = String::from("Rust est génial !");
    afficher_message(&message);  // ✅ Pas de transfert d’ownership
    println!("Longueur du message : {}", longueur(&message));  // ✅ Fonctionne toujours !

    // Explication :
    // - Ici, `message` est emprunté (`&String`) au lieu d’être transféré.
    // - On peut donc l’utiliser après l’appel.
    // - Rust garantit qu’on ne peut pas modifier `message` tant qu’il est emprunté.
}
