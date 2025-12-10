pub fn sum_array(arr: &[i32])-> i32 {
    arr.iter().sum()
}

fn main(){
    let ii = [1,2,3];
    println!("{:#?}", sum_array(&ii));
}

