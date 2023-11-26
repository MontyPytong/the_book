use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Ghiceste numarul!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
loop {

    println!("Te rog sa introduci numarul tau.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Nu am reusit sa citesc linia");

    let guess: u32 =match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };


    println!("Ai ghicit : {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("\x1b[93mPrea mic\x1b[0m"),
        Ordering::Greater => println!("\x1b[31mPrea mare\x1b[0m"),
        Ordering::Equal => {
            println!("\x1b[32mAi castigat\x1b[0m");
            break;
        },
           
    }
}


}