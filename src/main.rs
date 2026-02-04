use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::thread_rng;

fn generate_length() -> usize {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(18..22);
    //println!("random number is {}", length);
    length
}




fn main(){ 
    let length = generate_length();

    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    println!("{}", random_string);
}