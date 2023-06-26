// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!

// https://rust-analyzer.github.io/manual.html#unlinked-file
// i did the suggested quickfix and it inserted this line:
mod _quiz_2_blank;

// Put your function here!
fn calculate_price_of_apples(quantity_bought: u32) -> u32 {
    let mut price = 2;

    if quantity_bought <= 40 {
        quantity_bought * price
    } else {
        price = 1;
        quantity_bought * price
    }
}

// if the object is to be as concise as possible =>
fn concise(quantity_bought: u32) -> u32 {
    if quantity_bought <= 40 {
        quantity_bought * 2
    } else {
        quantity_bought
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
