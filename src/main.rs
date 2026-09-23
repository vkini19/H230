fn test_product(id: String){
    println!("Product ID: {}", id);
}

fn test_quantity(quantity: i32){
    print!("Quantity: {}", quantity);
}

fn main(){
    let product_id = String::from("P001");
    let quantity: i32 = 10;

    test_product(product_id);
    test_quantity(quantity);
}