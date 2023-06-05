fn arrays(){
    let arr_1: [i32; 4] = [1,2,3,4];

    println!("1st : {}", arr_1[0]);
    println!("length : {}", arr_1.len());

    let mut loop_idx = 0;
    loop{
        if arr_1[loop_idx] % 2 == 0{
            loop_idx +=1;
            continue;
        }
        if arr_1[loop_idx] == 4 {
            break;
        }
        println!("val : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    while loop_idx < arr_1.len(){
        println!("Arr : {}", arr_1[loop_idx]);
        loop_idx +=1;
    }

    for val in arr_1.iter() {
        println!("val : {}", val);
    }
}