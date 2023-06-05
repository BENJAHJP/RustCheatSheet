fn tuples(){
    let my_tuple: (u8, String, f64) = (47, "Benja".to_string(), 50_000_000);

    println!("NAme: {}", my_tuple.1);

    let(v1, v2, v3) = my_tuple;

    println!("Age: {}", v1);
}