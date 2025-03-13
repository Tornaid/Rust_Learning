// Déclare une variable mutable et incrémente sa valeur dans une boucle.
// Affiche la valeur après chaque incrémentation.

fn main() {
    let mut compteur = 0;

    for _ in 0..5 {
        compteur += 1;
        println!("Valeur actuelle de compteur : {}", compteur);
    }
}
