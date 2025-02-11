fn main() {
    let apple_price = 50;
    let apple_quantity = 2;
    let orange_price = 30;
    let orange_quantity = 4;
    calculate_expense(
        apple_price, 
        apple_quantity,
        orange_price,
        orange_quantity
    );
}

fn calculate_expense(
    apple_price: i32,
    apple_quantity: i32,
    orange_price: i32,
    orange_quantity: i32
) {
    let total_cost = apple_price*apple_quantity + orange_price*orange_quantity;
    println!("Total Cost is ${}", total_cost);
}
