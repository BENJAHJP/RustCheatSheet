use rand::Rng;

fn random(){
    let random_num = rand::thread_rng().gen_range(1..101);
    println!(random_num);
}