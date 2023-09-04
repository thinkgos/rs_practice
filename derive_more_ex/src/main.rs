use derive_more::{Display, From};

// 实现 struct Display trait
#[derive(Display, Debug)]
#[display(fmt = "Item: {}, Quantity: {}", item, qty)]
struct Order {
    item: String,
    qty: usize,
}

// 实现 enum Display trait
#[derive(Display)]
enum GroceryItem {
    #[display(fmt = "Bread slices: {}", _0)]
    Bread(usize),
    #[display(fmt = "Fruit")]
    Fruit,
    #[display(fmt = "Ounces of meat: {}", _0)]
    Meat(usize),
}

// 实现 from trait
#[derive(Debug, From)]
struct UserId(i32);

fn main() {
    let order = Order {
        item: "Coffee".to_string(),
        qty: 1,
    };
    println!("{:#?}", order);
    println!("{}", order);

    println!("{}", GroceryItem::Bread(1));
    println!("{}", GroceryItem::Fruit);
    println!("{}", GroceryItem::Meat(3));

    let user_id: UserId = 15.into();
    println!("{:?} {:?}", user_id, user_id);
}
