use std::collections::HashMap;

fn main() {
    let mut DB = Manager {
        user: vec![],
        all_order: HashMap::new(),
    };

    let user1 = User {
        email: "alice@gmail.com".to_string(),
    };

    let register_user = DB.register_user(user1);
    match register_user {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let order = Order { id: 32, amount: 50.0 };

    let create_order = DB.create_order(order, "alice@gmail.com");
    match create_order {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let print = DB.print_order("alice@gmail.com");
    match print {
        Ok(value) => (),
        Err(e) => println!("{}", e),
    };

}

struct User {
    email: String
}

struct Order {
    id: u32,
    amount: f64,
}

struct Manager {
    user: Vec<User>,
    all_order: HashMap<String, Vec<Order>>,
}
impl Manager {
    fn register_user(&mut self, user: User) -> Result<String, String> {
        let register = if self.user.iter().any(|u| u.email == user.email) {
            Err(format!("User allready registred"))
        } else {
            self.user.push(user);
            Ok("User register".to_string())
        };
        register
    }

    fn create_order(&mut self, order: Order, email: &str) -> Result<String, String> {
        let entry = self.all_order
        .entry(email.to_string())
        .or_insert_with(Vec::new);
    let exsists = entry
    .iter()
    .any(|o| o.id == order.id);

    let result = if exsists {
        Err("Order allready exsists".to_string())
    } else {
        entry.push(order);
        Ok("Order add".to_string())
    };
    result
    }

    fn print_order(self, email: &str) -> Result<(), String> {
        let user_orders = self.all_order
        .get(email)
        .ok_or_else(|| "User not make order yet".to_string())?;
        println!("Orders for alice@gmail.com:");
        for order in user_orders {
        println!("- Order {} : {}", order.id, order.amount);

        let sum: f64 = user_orders.iter()
        .map(|a| a.amount)
        .sum();
    println!("Total spent: {}", sum);
    }
    Ok(())
    }


}

