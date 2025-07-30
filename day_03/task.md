## 📦 Day 3 — Functions, Return, Tuples

### 🧠 Super Task — Warehouse & Shipping System

#### 🎯 Task Description

You're building a minimal backend logic for a warehouse. A customer places an order, and the system must evaluate the request and either reject it or confirm it with pricing.

#### 📥 Input Variables

```rust
let in_stock: u32 = 20;
let min_order: u32 = 5;
let requested: u32 = 12;
let is_vip: bool = true;

📋 Business Rules
1. If requested < min_order and user is not VIP → "Too small order"

2. If requested > in_stock → "Not enough stock"

3. Otherwise:

* Base price = 15.0 per item

* VIP users get a 10% discount

* Return result:

     * &str: status = "Order confirmed"

     * u32: items to ship

     * f64: total price (with discount if any)

🛠 Requirements
* Function: process_order(...)

* Params: in_stock: u32, min_order: u32, requested: u32, is_vip: bool

* Return: (&'static str, u32, f64)

* Example:

let result = process_order(20, 5, 12, true);
// → ("Order confirmed", 12, 162.0)

⚠️ Challenges
* Multiple intersecting conditions (e.g. both stock & VIP logic)

* Proper return types with mix of &str, u32, f64

* Must avoid unnecessary logic if early rejection

* Business-minded function structure — realistic backend thinking
