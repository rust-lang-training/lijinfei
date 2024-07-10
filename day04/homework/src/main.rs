use std::{iter::Map, collections::HashMap, fmt::Display, io::{stdin, Read}};
use rand::Rng;

const PRODUCT1: &'static str = include_str!("./product1.txt");
const PRODUCT2: &'static str = include_str!("./product2.txt");
const PRODUCT3: &'static str = include_str!("./product3.txt");
fn main() {
    let mut products = ProductRecord::gen_records();
    let mut login_info = ProductRecord::gen_login();
    // println!("{}", products);
    // print_products(&mut products);
    println!("------ 商品下单系统 ------");
    // 分类浏览商品
    // 根据名称查询商品
    // 下单。如果库存不足，则显示下单失败
    // 支付订单。如果余额不足，则显示支付失败
    // 统计自己订单的总金额
    loop {
        print_main_menu();
        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("请输入正确的选择");
        match choice.trim() {
            "1" => {
                print_products(&mut products);
            },
            "2" => {
                println!("请输入商品名称");
                let mut product_name = String::new();
                stdin().read_line(&mut product_name).unwrap();
                let mut product_name = product_name.trim();
                // let mut find_product = ProductRecord::new();
                products.values().for_each(|product| {
                    let result = product.iter().find(|pro| -> bool {
                        if pro.name == product_name {
                            true
                        } else {
                            false
                        }
                    });
                    match result {
                        Some(product) => {
                            println!("找到商品 {}", product);
                            println!("\n输入1继续下单");
                            let mut select = String::new();
                            stdin().read_line(&mut select).unwrap();
                            match select.trim() {
                                "1" => {
                                    let new_product = ProductRecord::new(&product.classify, product.id, &product.name, product.price, 1);
                                    if login_info.price >= new_product.price {
                                        if product.count > 0  {
                                            login_info.price = login_info.price - new_product.price.clone();
                                            login_info.records.push(new_product);
                                            println!("购买成功，当前余额为{}", login_info.price);
                                        } else {
                                            println!("库存不足");
                                        }
                                        
                                    } else {
                                        println!("余额不足")
                                    }
                                },
                                _ => {
                                    println!("请重新操作");
                                }
                            }
                        },
                        None => {
                            println!("未找到商品")
                        }
                    }
                })
            },
            "3" => {
                println!("当前个人信息为: {}, 余额: {}", login_info.name, login_info.price);
                println!("购买记录为：");
                login_info.records.iter().for_each(|product| {
                    println!("{}", product);
                })
            },
            "4" => {
                break;
            },
            _ => {
                println!("Unknow Option")
            }
        }
    }
}
fn print_products(products: &mut HashMap<String, Vec<ProductRecord>>) {
    products.iter().for_each(|product| {
        println!("当前分类为：{}", product.0);
        product.1.iter().for_each(|product| {
            println!("{}", product);
        });
    })
}
fn print_main_menu() {
    println!("输入1分类查看商品");
    println!("输入2根据名称查看商品");
    println!("输入3查看个人信息");
    println!("输入4退出");
}

struct ProductRecord {
    classify: String,
    id: u8,
    name: String,
    price: u8,
    count: u8
}
struct LoginInfo {
    name: String,
    price: u8,
    records: Vec<ProductRecord>
}
impl ProductRecord {
    fn new(classify: &str, id: u8, name: &str, price: u8, count: u8) -> Self {
        Self {
            classify: String::from(classify),
            id,
            name: String::from(name),
            price,
            count
        }
    }
    fn gen_records() -> HashMap<String, Vec<Self>> {
        let mut products = HashMap::<String, Vec<Self>>::new();
        let product1 = gen_product(PRODUCT1);
        let product2 = gen_product(PRODUCT2);
        let product3 = gen_product(PRODUCT3);
        products.insert(product1.0, product1.1);
        products.insert(product2.0, product2.1);
        products.insert(product3.0, product3.1);
        products
    }
    fn gen_login() -> LoginInfo {
        LoginInfo { name: String::from("大聪明"), price: 100u8, records: vec![] }
    }
}

impl Display for ProductRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03} {:6} {:>10} 价格为:{:<6}  库存为:{:<6}", 
            self.id,
            self.classify,
            self.name,
            self.price,
            self.count
        )
    }
}

fn gen_product(product: & 'static str) -> (String, Vec<ProductRecord>) {
    let mut result = product.split("\n");
    let mut id = 0u8;
    let mut classify = String::new();
    let mut results: Vec<ProductRecord> = result.filter(|s| !s.is_empty()).take(100).map(|str| {
        if id==0 {
            classify = String::from(str);
        }

        let mut rng = rand::thread_rng();
        let count = rng.gen_range(1..=10);
        let price = rng.gen_range(1..=100);
        id += 1;
        ProductRecord {
            id,
            classify: classify.clone(),
            name: String::from(str),
            count,
            price
        }
    }).collect();
    (classify, results.splice(1.., vec![]).collect())
}