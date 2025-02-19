fn main() {
    let apple_price:f64 = 50.0;
    let apple_quantity:f64 = 2.0;
    let orange_price:f64 = 30.0;
    let orange_quantity:f64 = 4.0;
    calculate_expense(
        apple_price, 
        apple_quantity,
        orange_price,
        orange_quantity
    );
}

fn calculate_expense(
    apple_price: f64,
    apple_quantity: f64,
    orange_price: f64,
    orange_quantity: f64
) {
    // Mutable to allow reassignment
    let mut total_cost:f64 = apple_price*apple_quantity + orange_price*orange_quantity;
    println!("Total Cost is ${total_cost}");

    total_cost = (total_cost as f64) * 0.9;
    println!("After discount applied, total Cost is ${total_cost}");
}
