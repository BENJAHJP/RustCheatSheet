fn if_expression(){
    let age: i32 = 8;

    if(age >=1) && (age<=18){
        println!("Important birthday");
    } else if (age == 21) || (age ==50){
        println!("IMportant birthday");
    } else if age >= 65{
        println!("Age is important");
    } else {
        println!("Not important");
    }

    let my_age: i32 = 47;
    let can_vote: bool = if my_age >= 18{
        true
    } else {
        false
    };

    println!("Can vote: {}", can_vote);
}