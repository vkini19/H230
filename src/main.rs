fn test_product(id: &String){
    println!("Product ID: {}", id);
}

fn test_quantity(quantity: i32){
    println!("Quantity: {}", quantity);
}

fn get_is_active(state: u32) -> bool {
    (state & (1 << 0)) != 0
}

fn get_is_admin(state: u32) -> bool {
    (state & (1 << 1)) != 0
}

fn get_group_id(state: u32) -> u32 {
    (state >> 2) & 0b1111111
}

fn get_endianness(state: u32) -> bool {
    (state & (1 << 9)) != 0
}

fn get_group_id_big_endian(state: u32) -> u32 {
    let group_id = get_group_id(state);
    let is_big_endian = get_endianness(state);

    if is_big_endian {
        group_id.to_be()
    } else {
        group_id
    }
}

fn main() {
    let product_id = String::from("P001");
    let quantity: i32 = 10;

    test_product(&product_id);
    test_quantity(quantity);

    println!("{}", product_id);
    
    let system_state: u32 = (1 << 0) | (5 << 2) | (1 << 9);

    println!("System state: {}", system_state);

    println!("Is active: {}", get_is_active(system_state));
    println!("Is admin: {}", get_is_admin(system_state));
    println!("Group ID: {}", get_group_id(system_state));
    println!("Big-endian flag: {}", get_endianness(system_state));

    let converted_group_id = get_group_id_big_endian(system_state);
    println!("Group ID after endian conversion: {}", converted_group_id);
}