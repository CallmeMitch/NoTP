extern crate rand;
extern crate std;

fn lire_entier() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Entier invalide")
}

fn main() {

    use rand::Rng;

    let secret_number = rand::rng().random_range(1..101);
    println!("Enter a number:");

    let mut input = lire_entier();
    while secret_number != input {

        if input < secret_number {
            println!("Plus grand !");
        } else {
            println!("Plus petit !");
        }

        input = lire_entier();

    }
    println!("Trouvé!");
}
