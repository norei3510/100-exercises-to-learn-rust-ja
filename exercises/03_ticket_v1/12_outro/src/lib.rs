// TODO: 新しい `Order` 型を定義してください。
//   `product_name`、`quantity`、`unit_price` の3つの情報を保持する必要があります。
//   商品名は空であってはならず、300バイトを超えてはなりません。
//   数量は0より厳密に大きくなければなりません。
//   単価はセント単位で、0より厳密に大きくなければなりません。
//   Order には、注文の合計金額を返す `total` という名前のメソッドを含める必要があります。
//   Order には、各フィールドに対するセッターとゲッターを提供する必要があります。
//
// 今回のテストは別の場所、`tests` フォルダにあります。
// `tests` フォルダは `cargo` 用の特別な場所です。ここは **統合テスト** を探す場所になります。
// ここでの「統合」には非常に具体的な意味があります。つまり、プロジェクトの **パブリックAPI** をテストするものです。
// 型やメソッドの可視性に注意する必要があります。統合
// テストでは、プライベートまたは `pub(crate)` の項目にアクセスできません。
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
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

    pub fn set_product_name(&mut self, product_name: String) {
        validate_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        validate_quantity(&quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_unit_price(&unit_price);
        self.unit_price = unit_price;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("Product name cannot be empty");
    }
    if product_name.len() > 300 {
        panic!("Product name cannot be longer than 300 bytes");
    }
}

fn validate_quantity(quantity: &u32) {
    if quantity == &0 {
        panic!("Quantity must be greater than zero");
    }
}

fn validate_unit_price(unit_price: &u32) {
    if unit_price == &0 {
        panic!("Unit price must be greater than zero");
    }
}
