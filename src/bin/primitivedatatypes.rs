pub  fn data_types() -> (u8 , f64 , bool , char){
    let r :u8 = 42;
    let b :f64 = 3.14;
    let c: bool = false; 
    let d : char = 'a';
    (r,b,c,d)
}

 fn main(){
    println!(" value:{:#?}", data_types())
}