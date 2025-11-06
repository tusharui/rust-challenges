pub fn calculate_area() -> u32 {
    let width = 2;
    let height = 3;
    prints_values(width, height);
    width * height
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}

// Add this main function to run the code
fn main() {
    let area = calculate_area();
    println!("The area is: {}", area);
}
