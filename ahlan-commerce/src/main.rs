#[derive(Clone)]
struct ProductId(String);

#[derive(Clone)]
pub struct Product {
    pub id:ProductId,
    pub title:String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool
}

pub struct ProductCreate {
    pub title: String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
}


pub struct Store{
    products:Vec<Product>
}

impl Store{
    pub fn new() -> Self {
        Self {
            products: Vec::new(),
        }
    }


    fn create_product(&mut self,input: ProductCreate) -> Product{
        let product =Product {
            id: ProductId(String::from("PROD-001")),
            title: input.title,
            handle: input.handle,
            price_cents: input.price_cents,
            inventory_quantity: input.inventory_quantity,
            published: input.published,
        };

        self.products.push(product.clone());

        return product ;
    }


    pub fn list_products(&self) -> Vec<Product> {
        self.products.clone()
    }
}
fn main() {
    let mut store = Store::new();

    let input = ProductCreate {
        title: String::from("Mac"),
        handle: String::from("mac"),
        price_cents: 500000,
        inventory_quantity: 7,
        published: true,
    };

    let product = store.create_product(input);

    println!("Created: {}", product.title);

    let products = store.list_products();

    for product in products {
        println!("Name : {} with price : {}",product.title,product.price_cents);
    }

    //println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_product() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Laptop"),
            handle: String::from("laptop"),
            price_cents: 50000,
            inventory_quantity: 10,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.title, "Laptop");
        assert_eq!(product.handle, "laptop");
        assert_eq!(product.price_cents, 50000);
        assert_eq!(product.inventory_quantity, 10);
        assert_eq!(product.published, true);
    }

    #[test]
    fn test_list_products() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Laptop"),
            handle: String::from("laptop"),
            price_cents: 50000,
            inventory_quantity: 10,
            published: true,
        };

        store.create_product(input);

        let products = store.list_products();

        assert_eq!(products.len(), 1);
        assert_eq!(products[0].title, "Laptop");
        assert_eq!(products[0].price_cents, 50000);
    }

    #[test]
    fn test_list_products_empty() {
        let store = Store::new();

        let products = store.list_products();

        assert_eq!(products.len(), 0);
    }

        
    #[test]
    fn test_create_multiple_products() {
        let mut store = Store::new();

        let input1 = ProductCreate {
            title: String::from("Laptop"),
            handle: String::from("laptop"),
            price_cents: 50000,
            inventory_quantity: 10,
            published: true,
        };

        let input2 = ProductCreate {
            title: String::from("Phone"),
            handle: String::from("phone"),
            price_cents: 30000,
            inventory_quantity: 5,
            published: false,
        };

        store.create_product(input1);
        store.create_product(input2);

        let products = store.list_products();

        assert_eq!(products.len(), 2);
        assert_eq!(products[0].title, "Laptop");
        assert_eq!(products[1].title, "Phone");
    }

    #[test]
    fn test_product_id_is_assigned() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Laptop"),
            handle: String::from("laptop"),
            price_cents: 50000,
            inventory_quantity: 10,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.id.0, "PROD-001");
    }

    #[test]
    fn test_create_product_with_zero_price() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Free Product"),
            handle: String::from("free-product"),
            price_cents: 0,
            inventory_quantity: 10,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.price_cents, 0);
    }

    #[test]
    fn test_create_product_with_zero_inventory() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Out of Stock"),
            handle: String::from("out-of-stock"),
            price_cents: 10000,
            inventory_quantity: 0,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.inventory_quantity, 0);
    }

    #[test]
    fn test_create_unpublished_product() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Draft Product"),
            handle: String::from("draft-product"),
            price_cents: 20000,
            inventory_quantity: 3,
            published: false,
        };

        let product = store.create_product(input);

        assert_eq!(product.published, false);
    }

    #[test]
    fn test_create_product_with_empty_title() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::new(),
            handle: String::from("empty-title"),
            price_cents: 10000,
            inventory_quantity: 5,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.title, "");
    }

    #[test]
    fn test_create_product_with_empty_handle() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Laptop"),
            handle: String::new(),
            price_cents: 10000,
            inventory_quantity: 5,
            published: true,
        };

        let product = store.create_product(input);

        assert_eq!(product.handle, "");
    }

    #[test]
    fn test_list_products_after_multiple_creations() {
        let mut store = Store::new();

        for i in 1..=5 {
            let input = ProductCreate {
                title: format!("Product {}", i),
                handle: format!("product-{}", i),
                price_cents: i * 1000,
                inventory_quantity: i,
                published: true,
            };

            store.create_product(input);
        }

        let products = store.list_products();

        assert_eq!(products.len(), 5);
        assert_eq!(products[0].title, "Product 1");
        assert_eq!(products[4].title, "Product 5");
    }

    #[test]
    fn test_list_products_does_not_modify_store() {
        let mut store = Store::new();

        let input = ProductCreate {
            title: String::from("Laptop"),
            handle: String::from("laptop"),
            price_cents: 50000,
            inventory_quantity: 10,
            published: true,
        };

        store.create_product(input);

        let products1 = store.list_products();
        let products2 = store.list_products();

        assert_eq!(products1.len(), 1);
        assert_eq!(products2.len(), 1);
    }




}

