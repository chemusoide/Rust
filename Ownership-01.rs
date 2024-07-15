fn main() {
    let owner1 = String::from("Hello, Rust"); // owner1 es el dueño del String

    let owner2 = owner1; // Transfiriendo la propiedad a owner2

    println!("{}", owner2); // Esto debería funcionar
    // println!("{}", owner1); // Esto causará un error, comenta esta línea para que el programa compile
}