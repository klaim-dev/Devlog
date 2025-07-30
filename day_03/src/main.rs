fn main() {
    let in_stock: f64 = 20.0;
    let price = 15.0;
    let min_order: f64 = 5.0;
    let requested: f64 = 12.0;
    let is_vip: bool = true;
    let (status, req, total_price) = process_order(price, in_stock, min_order, requested, is_vip);
    println!("{}, {}, {}", status, req, total_price);

}

fn process_order(price: f64, in_stock: f64, min_order: f64, requested: f64, is_vip: bool) -> (&'static str, f64, f64) {
    let enough = if in_stock > requested {
        true
    } else {
        false
    };

    let min = if min_order > requested && !is_vip {
        false
    } else {
        true
    };

    let total_price = if is_vip {
            requested * (price * 0.9)
        } else {
            price * requested
        };

    let status = if enough {
        if min {
            "Order confirmed"
        } else  {
            "Not enough stock"
        }
    } else {
        "Too small"
    };

    (status, requested, total_price)
}
