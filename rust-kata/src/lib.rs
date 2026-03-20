#[derive(Debug, Clone)]
pub struct Order {
    pub customer_type: String,
    pub subtotal_cents: i32,
    pub country: String,
    pub coupon_code: String,
    pub black_friday: bool,
}

pub fn calculate_total_cents(order: &Order) -> i32 {
    let subtotal = order.subtotal_cents;
    let customer_type = safe(&order.customer_type);
    let country = safe(&order.country);
    let coupon = safe(&order.coupon_code);

    let mut discount_percent = 0;
    discount_percent += base_discount_percent(&customer_type, subtotal);
    discount_percent += coupon_discount_percent(&coupon, &customer_type, subtotal);
    discount_percent += black_friday_discount_percent(order.black_friday, &customer_type);
    discount_percent = discount_percent.min(40);

    let discounted_subtotal = subtotal * (100 - discount_percent) / 100;
    let shipping_cents = shipping_cents(
        &country,
        &coupon,
        &customer_type,
        discounted_subtotal,
        order.black_friday,
    );
    let tax_percent = tax_percent(&country, &coupon, &customer_type);

    let tax_cents = discounted_subtotal * tax_percent / 100;
    let total = discounted_subtotal + shipping_cents + tax_cents;

    if total < 0 { 0 } else { total }
}

fn base_discount_percent(customer_type: &str, subtotal: i32) -> i32 {
    if customer_type == "vip" {
        15
    } else if customer_type == "premium" {
        if subtotal >= 10000 { 10 } else { 5 }
    } else if customer_type == "employee" {
        30
    } else if customer_type == "partner" {
        12
    } else {
        0
    }
}

fn coupon_discount_percent(coupon: &str, customer_type: &str, subtotal: i32) -> i32 {
    if coupon == "SAVE10" {
        if subtotal >= 5000 { 10 } else { 0 }
    } else if coupon == "VIPONLY" {
        if customer_type == "vip" { 5 } else { 0 }
    } else if coupon == "BULK" {
        if subtotal >= 20000 { 7 } else { 0 }
    } else if coupon == "PARTNER5" {
        if customer_type == "partner" && subtotal >= 12000 {
            5
        } else {
            0
        }
    } else {
        0
    }
}

fn black_friday_discount_percent(black_friday: bool, customer_type: &str) -> i32 {
    if !black_friday || customer_type == "employee" {
        0
    } else if customer_type == "partner" {
        3
    } else {
        5
    }
}

fn shipping_cents(
    country: &str,
    coupon: &str,
    customer_type: &str,
    discounted_subtotal: i32,
    black_friday: bool,
) -> i32 {
    let mut shipping_cents = base_shipping_cents(country);

    if black_friday && country == "US" {
        shipping_cents += 300;
    }

    if coupon == "FREESHIP" && discounted_subtotal >= 8000 {
        shipping_cents = 0;
    }

    if customer_type == "vip" && discounted_subtotal >= 15000 {
        shipping_cents = 0;
    }

    if customer_type == "premium" && discounted_subtotal >= 20000 {
        shipping_cents = 0;
    }

    if customer_type == "partner" && discounted_subtotal >= 15000 {
        shipping_cents = 0;
    }

    if customer_type == "employee" && country != "IT" {
        shipping_cents += 500;
    }

    shipping_cents
}

fn base_shipping_cents(country: &str) -> i32 {
    if country == "IT" {
        700
    } else if country == "DE" {
        900
    } else if country == "US" {
        1500
    } else {
        2500
    }
}

fn tax_percent(country: &str, coupon: &str, customer_type: &str) -> i32 {
    let mut tax_percent = if country == "IT" {
        22
    } else if country == "DE" {
        19
    } else if country == "US" {
        7
    } else {
        0
    };

    if customer_type == "vip" && country == "IT" {
        tax_percent = 20;
    }

    if coupon == "TAXFREE" && country != "IT" {
        tax_percent = 0;
    }

    tax_percent
}

fn safe(value: &str) -> String {
    value.trim().to_string()
}

#[cfg(test)]
mod tests;
