
// 📝 Exercice 3 : Emprunt mutable (`&mut`)

fn ajouter_exclamation(msg: &mut String) { // 📌 Emprunt mutable → Permet la modification
    msg.push_str("!");
}

fn main() {
    let mut message = String::from("Hello, Rust");
    
    ajouter_exclamation(&mut message); // ✅ On modifie le message
    ajouter_exclamation(&mut message); // ✅ On peut encore le modifier

    println!("{}", message); // ✅ "Hello, Rust!!"

    // Explication :
    // - Un emprunt mutable (`&mut String`) permet de modifier la variable sans en perdre la propriété.
    // - ⚠️ On ne peut **pas avoir plusieurs emprunts mutables en même temps**.
    // - Cela empêche les accès concurrents problématiques.
}
