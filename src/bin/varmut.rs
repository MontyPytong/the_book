// defining a constant
const TREI_ORE_IN_SECUNDE: u32 = 60 * 60 * 3;

fn main() {
    // COMPUND TYPES : tuple and array
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Valoarea lui x : {x}");
    println!("Valoarea lui y : {y}");
    println!("Valoarea lui z : {z}");

    let a = [1, 2, 3, 4, 5];
    let primul_element = a[0];
    println!("Primul element din array este {primul_element}");

    //DATA TYPES : char int float bool
    let guess: u32 = "42".parse().expect("Nu este un numar");

    //SHADOWING
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("Valoarea lui y in scope-ul interior este : {y}");
    }
    println!("Valoarea lui y este : {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Lungimea cuvantului spaces este : {spaces}");

    // pentru a putea defini alta valoare pentru variabila x trebuie sa o facem mutabila
    let mut x = 5;
    println!("Valoarea lui x este la primul print : {x}");
    x = 6;
    println!("Valoarea lui x este la al 2 lea print : {x}");
}
