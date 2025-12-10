pub fn math_operation(a:i32 , b:i32) ->(i32, i32 , i32, i32){
    let num = a+b;
    let sub = a-b;
    let multi = a*b;
    let divide = a/b;
    return (num, sub, multi, divide)


}

fn main (){
    let a = 4;
    let b = 16;
    println!("{:?}", math_operation(a,b))
}