fn main() {
    let mut age = 17;
    let mut status = "Teen";
    
    if age >= 20 {
        status = "Adult";
    } else {
        println!("You still youth 5!");
    }
    
    age += 1;

    println!("Final age: {}, Final status: {}", age, status);
}