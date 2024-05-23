#![allow(dead_code, unused_variables, unused_mut)]

// strutures are user defined vars to group related data together
#[derive(Debug)] // generate automatic Debug impl to print it
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    tax: f32,
}

#[derive(Debug)]
struct Product2 {
    name: String,
    price: f32,
    in_stock: bool,
}

// add functionality to Product2
impl Product2 {
    // common pattern to create "constructors"
    fn new(name: String, price: f32) -> Product2 {
        Product2 { // this is the actual only constructor of the struct
            name: name,
            price: price,
            in_stock: true,
        }
    }
    // associated functions (static)
    // associated to the struct itself so no reference to self is passed
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // reference to the current Product2 is passed as &self (immutable borrow)
    fn calculate_sale_tax(&self) -> f32 {
        self.price * Self::get_default_sales_tax() // to access the associated function use the name of the struct (or Self) and then ::
    }

    // to update fields it is necessary to define a mutable reference to self
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    // owned form of self can also be used if we want to destroy the struct after calling it
    // in order to prevent it, a cloned instance should be used instead
    // here makes sense if we do not want to use a book after it was bought
    fn buy(self) -> bool {
        println!("{} was bought", self.name);
        true
    }

    
}

fn main() {
    // create a new struct
    let mut book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
        tax: 0.0
    };

    println!("{:?}", book);

    // access its fields
    let price = book.price;
    
    // fields can be modified if the struct var is marked as mutable
    book.in_stock = false;
    println!("{:?}", book);

    // strucs are usually borrowed to functions in orde to keep using them later
    let sales_tax = calculate_sale_tax(&book);
    println!("Sales tax: {}", sales_tax);

    set_sale_tax(&mut book);
    println!("{:?}", book);

    let mut book2 = Product2 {
        name: String::from("Another book"),
        price: 30.15,
        in_stock: true
    };

    println!("Product2: {:?}", book2);
    println!("Product2 tax: {}", book2.calculate_sale_tax());

    book2.set_price(10.0);
    book2.buy();
    // book2.in_stock = false; <-- error because book2 was moved to buy method and then dropped

    let mut book3 = Product2::new(
        String::from("More books"),
        37.50
    );
    println!("New Product2: {:?}", book3);

}

// this method is linked to the Product and will be moved into it as an implementation block in Product2 struct
fn calculate_sale_tax(product: &Product) -> f32 {
    product.price * 0.1 // tax is 10%
}

// example to show modification by reference
fn set_sale_tax(product: &mut Product) {
    product.tax = product.price * 0.1;
}