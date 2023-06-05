fn strings(){
    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str("word");

    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("xyugdahgd  k j h gkj");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6: &str = &st5[0..6];

    println!("String length :{}", st6.len());

    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;

    for char in st8.bytes(){
        println!("{}", char);
        //unicode char
    }
}