mod market;
mod util;

use market::StockMarket::{Transaction, Order, TypeOfOperation, StockMarket};

use std::io::{stdin};
use std::convert::From;
use std::num::ParseFloatError;
use crate::market::StockMarket::{Currency, StockMarketMethod, USD, EURO};

struct User {
    amount: f64,
    price: f64,
    seller: String,
    currency: Currency,
    by_course: f64
}

#[macro_export]
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

    loop {
        println!("Какую операцию вы хотите провести?\n\
            Введите 1 что-бы добавить ордер на лимитную заявку\n\
            Введите 2 что-бы просмотреть предложение\n\
            Введите 3 что-бы получить разница между минимальной ценой продажи и максимальной ценой покупки\n\
            Введите 4 что-бы просмотреть транзакции\n\
            Введите 5 для просмотра данных в режиме dev (просто выводит все векторы и структуры)
        ");

        operation.clear();
        stdin().read_line(&mut operation).unwrap();
        match operation.trim().parse::<u64>() {
            Ok(1) => {
                let mut type_of_operation = String::from("");
                sell_or_buy(&mut type_of_operation, &mut stock_market);
            },
            Ok(2) => {
                get_offer(&stock_market.order);
            },
            Ok(3) => {
                stock_market.get_spread();
            },
            Ok(4) => {
                if stock_market.transaction.len() == 0 {
                    println!("Транзакций пока не было");
                }
                for el in stock_market.transaction.iter() {
                    println!("Продовец: {}\nПокупатель: {}\nПроданная сумма: {}\nПродано по цене: {}\n", el.seller, el.buyer, el.amount, el.price);
                }
            },
            Ok(5) => {
                println!("Transaction: {:#?}\nOrder: {:#?}", stock_market.transaction, stock_market.order)
            }
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
            println!("Что вы хотите купить?\nВведите 1 что-бы купить USD\nВведите 2 что-бы купить EURO\n");
        } else if *type_operation == TypeOfOperation::Sell {
            println!("Что вы хотите продать?\nВведите 1 что-бы продать USD\nВведите 2 что-бы продать EURO\n");
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
                let user = data_about_order(TypeOfOperation::Buy);

                StockMarket::push_order(stock_market, Order {
                    id: stock_market.order[stock_market.order.len() - 1].id + 1,
                    type_operation: TypeOfOperation::Buy,
                    amount: user.amount,
                    price: user.price,
                    seller: String::from(user.seller.trim()),
                    currency: user.currency,
                    by_course: user.by_course as f32
                });

                println!("Ваше предложение было добавлено!\nПросмотрите проведённые транзакции в разделе 4, если мы для вас нашли выгодную сделку то вы увидите это в транзакциях\n\n");
                stock_market.process();
                break;
            },

            Ok(2) => {
                let user = data_about_order(TypeOfOperation::Sell);

                StockMarket::push_order(stock_market, Order {
                    id: stock_market.order[stock_market.order.len() - 1].id + 1,
                    type_operation: TypeOfOperation::Sell,
                    amount: user.amount,
                    price: user.price,
                    seller: String::from(user.seller.trim()),
                    currency: user.currency,
                    by_course: user.by_course as f32
                });

                println!("Ваше предложение было добавлено!\n\n");
                stock_market.process();
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

fn data_about_order(type_operation: TypeOfOperation) -> User {
    let mut seller = String::from("");
    let mut currency_type = String::from("");
    let mut amount = String::from("");

    let currency_sell = trading_currency(&mut currency_type, &type_operation);
    let price_pir_unit = what_price(currency_sell);

    if type_operation == TypeOfOperation::Buy {
        println!("\nВведите ваше имя\n");
        stdin().read_line(&mut seller).unwrap();

        loop {
            println!("\nСколько {:?} вы хотите купить?\n", currency_sell);
            amount.clear();
            stdin().read_line(&mut amount).unwrap();
            match amount.trim().parse::<f64>() {
                Ok(num) => {
                    break
                }
                Err(_) => {
                    println!("Некорректное число введите занова!");
                }
            }
        }

    } else {
        println!("\nВведите ваше имя\n");
        stdin().read_line(&mut seller).unwrap();

        println!("\nСколько {:?} вы хотите продать?\n", currency_sell);
        stdin().read_line(&mut amount).unwrap();
    };

    let amount_to_f64: f64 = amount.trim().parse::<f64>().unwrap();

    User {
        amount: amount_to_f64,
        price: (price_pir_unit * amount_to_f64 * 100.0).round() / 100.0,
        seller,
        currency: currency_sell,
        by_course: price_pir_unit
    }
}

fn get_offer(stock_market: &Vec<Order>) {
    loop {
        let mut offer = String::from("");
        println!("\n\n1 просмотреть все предложение\n2 просмотреть что доступно в продаже\n3 просмотреть что доступно в покупке\n0 что-бы вернуться назад\n\n");
        stdin().read_line(&mut offer).unwrap();

        match offer.trim().parse::<u64>() {
            Ok(0) => {
                break
            }
            Ok(1) => {
                if stock_market.len() == 0 {
                    println!("\nНе чего не найдено\n")
                } else {
                    let mut transaction_number = String::from("");
                    helper_for((*stock_market.iter().collect::<Vec<&Order>>()).to_vec());
                    println!("\n0 вернуться назад\n");
                    stdin().read_line(&mut transaction_number).unwrap();

                    match transaction_number.trim().parse::<usize>() {
                        Ok(0) => {
                            break
                        }
                        _ => {
                            println!("\nчто-то не то\n");
                        }
                        Err(e) => {
                            println!("\nВведены не коррктные символы\n");
                        }
                    }
                }
            },
            Ok(2) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Sell).collect();

                if filter_by_type_operation.len() == 0{
                    println!("\nНе чего не найдено\n")
                } else {
                    helper_for(filter_by_type_operation);
                }

            },
            Ok(3) => {
                let filter_by_type_operation: Vec<&Order> = stock_market.iter().map(|of| of).filter(|el| el.type_operation == TypeOfOperation::Buy).collect();

                if filter_by_type_operation.len() == 0 {
                    println!("\nНе чего не найдено\n")
                } else {
                    helper_for(filter_by_type_operation);
                }
            },
            _ => {
                println!("\nВведите цифру в диапазоне 0-3\n");
                continue;
            },
            Err(e) => {
                println!("\nВведите корректные символы\n");
                continue;
            }
        }
    }
}

fn helper_for(filter_by_type_operation: Vec<&Order>) {
    for offer in filter_by_type_operation {
        println!("Номер сделки: {}\nИмя пользователя: {}\n{} {:?}\
        \n{} {} {:?}\nЦена: {} {:?}\nПо курсу: {} {:?} за 1 {:?}\n",
            offer.id,
            offer.seller,
            either!(offer.type_operation == TypeOfOperation::Buy => "Покупает: "; "Продаёт: "),
            offer.currency,
            either!(offer.type_operation == TypeOfOperation::Buy => "Купит: "; "Продаст: "),
            offer.amount,
            offer.currency,
            offer.price,
            either!(offer.currency == Currency::USD => Currency::EURO; Currency::USD),
            offer.by_course,
            either!(offer.currency == Currency::USD => Currency::EURO; Currency::USD),
            offer.currency,
        )
    }
}

fn what_price(currency: Currency) -> f64 {
    let mut price = String::from("");
    println!("При каком курсе {:?} в {:?} открывать ордер?\nПример: 1.04\nТоесть вы покупаете 1 {:?} За 1.04 {:?} ",
             either!(currency == Currency::USD => Currency::EURO; Currency::USD),
             currency,
             currency,
             either!(currency == Currency::USD => Currency::EURO; Currency::USD),
    );


    loop {
        price.clear();
        stdin().read_line(&mut price).unwrap();
        match price.trim().parse::<f64>() {
            Ok(num) => {
                return num
            }
            Err(e) => {
                println!("некорректное число");
                continue;
            }
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("My type: {}", std::any::type_name::<T>())
}

fn test_data() -> Box<[Order; 11]> {
    return Box::new([Order {
        id: 1,
        type_operation: TypeOfOperation::Sell,
        amount: 1296.0,
        price: 1290.0,
        seller: "Richard".to_string(),
        currency: Currency::USD,
        by_course: (1290.0_f32 / 1296.0_f32 * 100.0).round() / 100.0
    },
    Order {
        id: 2,
        type_operation: TypeOfOperation::Sell,
        amount: 542.0,
        price: 552.84,
        seller: "Imil".to_string(),
        currency: Currency::USD,
        by_course: (552.84_f32 / 542.0_f32 * 100.0).round() / 100.0
    },
        Order {
        id: 3,
        type_operation: TypeOfOperation::Buy,
        amount: 630.0,
        price: 612.49,
        seller: "Oskar".to_string(),
        currency: Currency::EURO,
        by_course: (630.0_f32 / 612.49_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 4,
        type_operation: TypeOfOperation::Buy,
        amount: 423.53,
        price: 432.0,
        seller: "John".to_string(),
        currency: Currency::USD,
        by_course: (1200.0 / 1231.0_f32 * 1000.0).round() / 1000.0
    },
        Order {
        id: 5,
        type_operation: TypeOfOperation::Sell,
        amount: 321.0,
        price: 327.42,
        seller: "Kiril".to_string(),
        currency: Currency::USD,
        by_course: (327.42_f32 / 327.01_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 6,
        type_operation: TypeOfOperation::Sell,
        amount: 30.0,
        price: 30.0,
        seller: "Ivan".to_string(),
        currency: Currency::EURO,
        by_course: (30.0_f32 / 30.0_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 7,
        type_operation: TypeOfOperation::Sell,
        amount: 5000.0,
        price: 5200.0,
        seller: "Matvey".to_string(),
        currency: Currency::USD,
        by_course: (5200.0_f32 / 5000.0_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 8,
        type_operation: TypeOfOperation::Buy,
        amount: 2314.0,
        price: 2239.67,
        seller: "Sveta".to_string(),
        currency: Currency::EURO,
        by_course: (2314.0 / 2239.67_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 9,
        type_operation: TypeOfOperation::Buy,
        amount: 322.0,
        price: 312.0,
        seller: "Diana".to_string(),
        currency: Currency::EURO,
        by_course: (322.0 / 312.0_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 10,
        type_operation: TypeOfOperation::Sell,
        amount: 716.15,
        price: 702.0,
        seller: "Sofa".to_string(),
        currency: Currency::EURO,
        by_course: (716.15_f32 / 702.0_f32 * 1000.0).round() / 1000.0
    }, Order {
        id: 11,
        type_operation: TypeOfOperation::Buy,
        amount: 17.15,
        price: 20.0,
        seller: "Aleksandr".to_string(),
        currency: Currency::USD,
        by_course: (20.0_f32 / 17.15_f32 * 1000.0).round() / 1000.0
    }
    ]);

}