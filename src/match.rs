fn matching(){
    let age2: i32 = 8;

    match age2{
        1..=18 => println!("Important"),
        21 | 50 => println!("Important"),
        65..=i32::MAX => println!("Important"),
        _ => println!("Important"),
    };

    let my_age:i32 = 18;
    let voting_age:i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote")
    };

}