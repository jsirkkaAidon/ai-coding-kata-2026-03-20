use super::{calculate_total_cents, Order};

fn order(
    customer_type: &str,
    subtotal_cents: i32,
    country: &str,
    coupon_code: &str,
    black_friday: bool,
) -> Order {
    Order {
        customer_type: customer_type.to_string(),
        subtotal_cents,
        country: country.to_string(),
        coupon_code: coupon_code.to_string(),
        black_friday,
    }
}

#[test]
fn example_regular_it_total_matches_readme() {
    let input = order("regular", 10000, "IT", "", false);
    assert_eq!(calculate_total_cents(&input), 12900);
}

#[test]
fn example_premium_de_save10_total_matches_readme() {
    let input = order("premium", 10000, "DE", "SAVE10", false);
    assert_eq!(calculate_total_cents(&input), 10420);
}

#[test]
fn example_vip_it_viponly_total_matches_readme() {
    let input = order("vip", 18000, "IT", "VIPONLY", false);
    assert_eq!(calculate_total_cents(&input), 17980);
}

#[test]
fn employee_black_friday_gets_no_extra_five_discount() {
    let input = order("employee", 10000, "US", "", true);
    assert_eq!(calculate_total_cents(&input), 9790);
}

#[test]
fn discount_is_capped_to_forty_percent() {
    let input = order("employee", 30000, "DE", "SAVE10", false);
    assert_eq!(calculate_total_cents(&input), 22820);
}

#[test]
fn free_ship_coupon_uses_discounted_subtotal_threshold() {
    let below = order("regular", 7900, "US", "FREESHIP", false);
    let above = order("regular", 8000, "US", "FREESHIP", false);

    assert_eq!(calculate_total_cents(&below), 9953);
    assert_eq!(calculate_total_cents(&above), 8560);
}

#[test]
fn black_friday_us_adds_shipping_surcharge() {
    let input = order("regular", 10000, "US", "", true);
    assert_eq!(calculate_total_cents(&input), 11965);
}

#[test]
fn employee_shipping_penalty_applies_outside_it() {
    let input = order("employee", 10000, "DE", "", false);
    assert_eq!(calculate_total_cents(&input), 9730);
}

#[test]
fn taxfree_removes_tax_outside_it_only() {
    let de = order("regular", 10000, "DE", "TAXFREE", false);
    let it = order("regular", 10000, "IT", "TAXFREE", false);

    assert_eq!(calculate_total_cents(&de), 10900);
    assert_eq!(calculate_total_cents(&it), 12900);
}

#[test]
fn vip_it_has_special_tax_rate() {
    let input = order("vip", 10000, "IT", "", false);
    assert_eq!(calculate_total_cents(&input), 10900);
}

#[test]
fn safe_trims_customer_country_and_coupon() {
    let input = order("  vip  ", 18000, "  IT  ", "  VIPONLY  ", false);
    assert_eq!(calculate_total_cents(&input), 17980);
}

#[test]
fn vip_and_premium_free_shipping_thresholds_apply() {
    let vip = order("vip", 20000, "DE", "", false);
    let premium = order("premium", 25000, "DE", "", false);

    assert_eq!(calculate_total_cents(&vip), 20230);
    assert_eq!(calculate_total_cents(&premium), 26775);
}

#[test]
fn partner_has_base_twelve_percent_discount() {
    let input = order("partner", 10000, "XX", "", false);
    assert_eq!(calculate_total_cents(&input), 11300);
}

#[test]
fn partner_gets_free_shipping_at_discounted_subtotal_threshold() {
    let below = order("partner", 17000, "XX", "", false);
    let at_or_above = order("partner", 17100, "XX", "", false);

    assert_eq!(calculate_total_cents(&below), 17460);
    assert_eq!(calculate_total_cents(&at_or_above), 15048);
}

#[test]
fn partner5_coupon_applies_only_for_partner_and_min_subtotal() {
    let partner_below = order("partner", 11900, "XX", "PARTNER5", false);
    let partner_ok = order("partner", 12000, "XX", "PARTNER5", false);
    let regular = order("regular", 12000, "XX", "PARTNER5", false);

    assert_eq!(calculate_total_cents(&partner_below), 12972);
    assert_eq!(calculate_total_cents(&partner_ok), 12460);
    assert_eq!(calculate_total_cents(&regular), 14500);
}

#[test]
fn partner_black_friday_bonus_is_three_percent_not_five() {
    let input = order("partner", 10000, "XX", "", true);
    assert_eq!(calculate_total_cents(&input), 11000);
}
