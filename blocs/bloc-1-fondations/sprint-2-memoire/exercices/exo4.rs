
// 📝 Exercice 4 : Lifetimes (`'a`)

fn plus_long<'a>(s1: &'a str, s2: &'a str) -> &'a str { 
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    let string1 = String::from("Rust");
    let string2 = String::from("Langage");

    let resultat = plus_long(&string1, &string2);
    println!("Le plus long est : {}", resultat);

    // Explication :
    // - `'a` est un **lifetime** qui garantit que la valeur retournée
    //   a une durée de vie au moins aussi longue que `s1` et `s2`.
    // - Sans le lifetime, Rust ne saurait pas si `resultat` est valide après l'appel.
}
