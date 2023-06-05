fn casting(){
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int2_u8 as u32) + (int_u8 as u32);
}