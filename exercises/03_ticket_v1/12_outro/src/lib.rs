pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    pub fn new(pn: String, qty: u32, up: u32) -> Self {
        if pn.is_empty() || pn.len() > 300 {
            panic!("Invalid product name");
        }

        if qty == 0 {
            panic!("Quantity is zero!")
        }

        if up == 0 {
            panic!("Unit price is zero!")
        }

        Self {
            product_name: pn,
            quantity: qty,
            unit_price: up
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, pn: String) {
        self.product_name = pn
    }

    pub fn set_quantity(&mut self, qty: u32) {
        self.quantity = qty
    }

    pub fn set_unit_price(&mut self, up: u32) {
        self.unit_price = up
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
    
}

// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
