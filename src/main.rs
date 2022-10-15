mod market;

use market::StockMarket::{Transaction, Order, TypeOfOperation, StockMarket};

use std::io;
use std::io::{stdin};
use std::num::ParseIntError;
use std::convert::From;
use std::fmt::Debug;
use crate::market::StockMarket::{Currency, StockMarketMethod};

struct User {
    amount: u64,
    price: u64,
    seller: String,
    currency: Currency
}

macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}

fn main() {
    let mut operation = String::from("");
    let mut stock_market = StockMarket::new();

    let data = test_data();

    for offer in data.iter() {
        stock_market.push_order(offer.clone());
    }

    // data.iter().map(move |offer| stock_market.push_order(*offer));

    loop {
        println!("Какую операцию вы хотите провести?\n\
            Введите 1 что-бы добавить ордер\n\
            Введите 2 что-бы просмотреть предложение\n\
            Введите 3 что-бы получить разница между минимальной ценой продажи и максимальной ценой покупки
        ");
        operation.clear();
        stdin().read_line(&mut operation).unwrap();
        match operation.trim().parse::<u64>() {
            Ok(1) => {
                let mut type_of_operation = String::from("");
                sell_or_buy(&mut type_of_operation, &mut stock_market);
            },
            Ok(2) => {
                let mut type_of_operation = String::from("");
                get_offer(&stock_market.order);
            },
            // Ok(3) => {
            //     println!("200");
            // },
            Err(e) => {
                println!("Введены не коррктные символы")
            },
            _ => {
                println!("Введите число в диопазоне 1-3!")
            }
        }
    }


}

fn trading_currency(currency: &mut String, type_operation: &TypeOfOperation) -> Currency {
    'valid_type: loop {
        if *type_operation == TypeOfOperation::Buy {
            println!("Что вы хотите купить?\nВведите 1 что-бы купить USD\nВведите 2 что-бы купить EURO");
        } else if *type_operation == TypeOfOperation::Sell {
            println!("Что вы хотите продать?\nВведите 1 что-бы продать USD\nВведите 2 что-бы продать EURO");
        }

         currency.clear();
         stdin().read_line(currency).unwrap();

         match currency.trim().parse::<u64>() {
            Ok(1) => {
                return Currency::USD;
            },
            Ok(2) => {
                return Currency::EURO;
            },
            Err(e) => {
                 println!("Вы ввели не коррктное символы");
                 continue
            },
            _ => {
                 println!("выберите в диапозоне 1-2");
                 continue
            }
        };
    }
}

fn sell_or_buy(type_of_operation: &mut String, stock_market: &mut StockMarket) {
    'validation_sell_buy: loop {
        println!("Введите 1 что-бы добавить ордер на покупку\nВведите 2 что-бы добавить ордер на продажу\n\n");
        type_of_operation.clear();
        stdin().read_line(type_of_operation).unwrap();

        match type_of_operation.trim().parse::<u64>() {
            Ok(1) => {
                let user = date_or_order(TypeOfOperation::Buy);

                StockMarket::push_order(stock_market, Order {
                    id: stock_market.order[stock_market.order.len() - 1].id + 1,
                    type_operation: TypeOfOperation::Buy,
                    amount: user.amount,
                    price: user.price,
                    seller: String::from(user.seller.trim()),
                    currency: user.currency,
                });

                println!("Ваше предложение было добавлено!\n\n");
                break;
            },

            Ok(2) => {
                let user = date_or_order(TypeOfOperation::Sell);

                StockMarket::push_order(stock_market, Order {
                    id: stock_market.order[stock_market.order.len() - 1].id + 1,
                    type_operation: TypeOfOperation::Sell,
                    amount: user.amount,
                    price: user.price,
                    seller: String::from(user.seller.trim()),
                    currency: user.currency,
                });

                println!("Ваше предложение было добавлено!\n\n");
                break
            },
            Err(e) => {
                 println!("Введены не коррктные символы");
                 continue
            },
            _ => {
                println!("Введите число в диопазоне 1-2!");
                continue
            }
       }
    }
}

fn date_or_order(type_operation: TypeOfOperation) -> User {
    let mut seller = String::from("");
    let mut currency_type = String::from("");
    let mut amount = String::from("");
    let mut price = String::from("");

    let currency_sell = trading_currency(&mut currency_type, &type_operation);

    if type_operation == TypeOfOperation::Buy {
        println!("Введите ваше имя");
        stdin().read_line(&mut seller).unwrap();

        println!("Сколько {:?} вы хотите купить?", currency_sell);
        stdin().read_line(&mut amount).unwrap();

        println!("Назавите свою цену, она будет в {:?}", either!(currency_sell == Currency::USD => Currency::EURO; Currency::USD));
        stdin().read_line(&mut price).unwrap();
    } else {
        println!("Введите ваше имя");
        stdin().read_line(&mut seller).unwrap();

        println!("Сколько {:?} вы хотите продать?", currency_sell);
        stdin().read_line(&mut amount).unwrap();

        println!("За сколько вы хотите продать {:?}?", either!(currency_sell == Currency::USD => Currency::EURO; Currency::USD));
        stdin().read_line(&mut price).unwrap();
    };

    let amount_to_u64 = amount.trim().parse::<u64>().unwrap();
    let price_to_u64 = price.trim().parse::<u64>().unwrap();

    User {
        amount: amount_to_u64,
        price: price_to_u64,
        seller,
        currency: currency_sell
    }
}

fn get_offer(stock_market: &Vec<Order>) {
    loop {
        let mut offer = String::from("");
        println!("1 просмотреть все предложение\n2 просмотреть что доступно в продаже\n3 просмотреть что доступно в покупке\n0 что-бы вернуться назад\n\n");
        stdin().read_line(&mut offer).unwrap();

        match offer.trim().parse::<u64>() {
            Ok(0) => {
                break
            }
            Ok(1) => {
                if stock_market.len() == 0 {
                    println!("Не чего не найдено")
                } else {
                    helper_for((*stock_market.iter().collect::<Vec<&Order>>()).to_vec());
                }
            },
            Ok(2) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Sell).collect();

                if filter_by_type_operation.len() == 0{
                    println!("Не чего не найдено")
                } else {
                    helper_for(filter_by_type_operation);
                }

            },
            Ok(3) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Buy).collect();

                if filter_by_type_operation.len() == 0 {
                    println!("Не чего не найдено")
                } else {
                    helper_for(filter_by_type_operation);
                }
            },
            _ => {
                println!("Введите цифру в диапазоне 1-3");
                continue;
            },
            Err(e) => {
                println!("Введите корректные символы");
                continue;
            }
        }
    }
}

fn helper_for(filter_by_type_operation: Vec<&Order>) {
    for offer in filter_by_type_operation {
        println!("Номер сделки: {}\nИмя пользователя: {}\nПродаваемая валюта: {:?}\nКоличество выведенное на продажу: {}\nЦена: {}\n", offer.id, offer.seller, offer.currency, offer.amount, offer.price)
    }
}

fn print_type_of<T>(_: &T) {
    println!("My type::::: {}", std::any::type_name::<T>())
}

fn test_data() -> Box<[Order; 10]> {
    return Box::new([Order {
        id: 1,
        type_operation: TypeOfOperation::Sell,
        amount: 500,
        price: 510,
        seller: "Imil".to_string(),
        currency: Currency::USD
    }, Order {
        id: 2,
        type_operation: TypeOfOperation::Buy,
        amount: 630,
        price: 600,
        seller: "Oskar".to_string(),
        currency: Currency::EURO
    }, Order {
        id: 3,
        type_operation: TypeOfOperation::Buy,
        amount: 1200,
        price: 1300,
        seller: "John".to_string(),
        currency: Currency::USD
    }, Order {
        id: 4,
        type_operation: TypeOfOperation::Sell,
        amount: 737,
        price: 400,
        seller: "Kiril".to_string(),
        currency: Currency::USD
    }, Order {
        id: 5,
        type_operation: TypeOfOperation::Sell,
        amount: 30,
        price: 28,
        seller: "Ivan".to_string(),
        currency: Currency::EURO
    }, Order {
        id: 6,
        type_operation: TypeOfOperation::Sell,
        amount: 5000,
        price: 5200,
        seller: "Matvey".to_string(),
        currency: Currency::USD
    }, Order {
        id: 7,
        type_operation: TypeOfOperation::Buy,
        amount: 2314,
        price: 2250,
        seller: "Sveta".to_string(),
        currency: Currency::EURO
    }, Order {
        id: 8,
        type_operation: TypeOfOperation::Buy,
        amount: 322,
        price: 312,
        seller: "Diana".to_string(),
        currency: Currency::EURO
    }, Order {
        id: 9,
        type_operation: TypeOfOperation::Sell,
        amount: 716,
        price: 702,
        seller: "Sofa".to_string(),
        currency: Currency::EURO
    }, Order {
        id: 10,
        type_operation: TypeOfOperation::Buy,
        amount: 17,
        price: 20,
        seller: "Diana".to_string(),
        currency: Currency::USD
    }]);
}
























// fn convert_string_u64(amount: &Currency, mut quantity: String) -> u64 {
//     let res: u64 = 'validation_quantity: loop {
//         println!("Сколько {:?} вы хотите купить?", amount);
//         stdin().read_line(&mut quantity).unwrap();
//
//         match quantity.trim().parse::<u64>().unwrap() {
//             u64 => {
//                 break u64;
//             },
//             _ => {
//                 println!("Введите правильное число");
//                 continue;
//             },
//             Err(e) => {
//                 continue;
//             }
//         }
//     };
//
//     return res
// }