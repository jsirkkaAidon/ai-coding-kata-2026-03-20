#[derive(Debug, Clone)]
pub struct Order {
    pub customer_type: String,
    pub subtotal_cents: i32,
    pub country: String,
    pub coupon_code: String,
    pub black_friday: bool,
}

pub fn calculate_total_cents(order: &Order) -> i32 {
    let context = PricingContext::from(order);
    let discount_percent = DiscountRules::new(&context).total_discount_percent();
    let discounted_subtotal = subtotal_after_discount(context.subtotal_cents, discount_percent);
    let shipping_cents = ShippingRules::new(&context).shipping_cents(discounted_subtotal);
    let tax_percent = TaxRules::new(&context).tax_percent();
    let tax_cents = discounted_subtotal * tax_percent / 100;
    let total = discounted_subtotal + shipping_cents + tax_cents;

    if total < 0 { 0 } else { total }
}

struct PricingContext {
    customer_type: CustomerType,
    subtotal_cents: i32,
    country: Country,
    coupon: Coupon,
    black_friday: bool,
}

impl PricingContext {
    fn from(order: &Order) -> Self {
        Self {
            customer_type: CustomerType::from(safe(&order.customer_type).as_str()),
            subtotal_cents: order.subtotal_cents,
            country: Country::from(safe(&order.country).as_str()),
            coupon: Coupon::from(safe(&order.coupon_code).as_str()),
            black_friday: order.black_friday,
        }
    }
}

struct DiscountRules<'a> {
    context: &'a PricingContext,
}

impl<'a> DiscountRules<'a> {
    fn new(context: &'a PricingContext) -> Self {
        Self { context }
    }

    fn total_discount_percent(&self) -> i32 {
        let raw_discount = self.base_customer_discount()
            + self.coupon_discount()
            + self.black_friday_discount();

        raw_discount.min(40)
    }

    fn base_customer_discount(&self) -> i32 {
        self.context
            .customer_type
            .base_discount_percent(self.context.subtotal_cents)
    }

    fn coupon_discount(&self) -> i32 {
        self.context.coupon.discount_percent(self.context)
    }

    fn black_friday_discount(&self) -> i32 {
        if self.context.black_friday {
            self.context.customer_type.black_friday_discount_percent()
        } else {
            0
        }
    }
}

struct ShippingRules<'a> {
    context: &'a PricingContext,
}

impl<'a> ShippingRules<'a> {
    fn new(context: &'a PricingContext) -> Self {
        Self { context }
    }

    fn shipping_cents(&self, discounted_subtotal: i32) -> i32 {
        let mut shipping_cents = self.context.country.base_shipping_cents();

        if self.context.black_friday && self.context.country == Country::UnitedStates {
            shipping_cents += 300;
        }

        if self.context.coupon == Coupon::FreeShip && discounted_subtotal >= 8000 {
            shipping_cents = 0;
        }

        if let Some(threshold) = self.context.customer_type.free_shipping_threshold_cents()
            && discounted_subtotal >= threshold
        {
            shipping_cents = 0;
        }

        if self.context.customer_type == CustomerType::Employee
            && self.context.country != Country::Italy
        {
            shipping_cents += 500;
        }

        shipping_cents
    }
}

struct TaxRules<'a> {
    context: &'a PricingContext,
}

impl<'a> TaxRules<'a> {
    fn new(context: &'a PricingContext) -> Self {
        Self { context }
    }

    fn tax_percent(&self) -> i32 {
        if self.context.coupon == Coupon::TaxFree && self.context.country != Country::Italy {
            return 0;
        }

        if self.context.customer_type == CustomerType::Vip && self.context.country == Country::Italy {
            return 20;
        }

        self.context.country.base_tax_percent()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CustomerType {
    Vip,
    Premium,
    Employee,
    Regular,
    New,
    Partner,
    Unknown,
}

impl CustomerType {
    fn from(value: &str) -> Self {
        match value {
            "vip" => Self::Vip,
            "premium" => Self::Premium,
            "employee" => Self::Employee,
            "regular" => Self::Regular,
            "new" => Self::New,
            "partner" => Self::Partner,
            _ => Self::Unknown,
        }
    }

    fn base_discount_percent(self, subtotal_cents: i32) -> i32 {
        match self {
            Self::Vip => 15,
            Self::Premium if subtotal_cents >= 10_000 => 10,
            Self::Premium => 5,
            Self::Employee => 30,
            Self::Partner => 12,
            Self::Regular | Self::New | Self::Unknown => 0,
        }
    }

    fn black_friday_discount_percent(self) -> i32 {
        match self {
            Self::Employee => 0,
            Self::Partner => 3,
            Self::Vip | Self::Premium | Self::Regular | Self::New | Self::Unknown => 5,
        }
    }

    fn free_shipping_threshold_cents(self) -> Option<i32> {
        match self {
            Self::Vip => Some(15_000),
            Self::Premium => Some(20_000),
            Self::Partner => Some(15_000),
            Self::Employee | Self::Regular | Self::New | Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Country {
    Italy,
    Germany,
    UnitedStates,
    Other,
}

impl Country {
    fn from(value: &str) -> Self {
        match value {
            "IT" => Self::Italy,
            "DE" => Self::Germany,
            "US" => Self::UnitedStates,
            _ => Self::Other,
        }
    }

    fn base_shipping_cents(self) -> i32 {
        match self {
            Self::Italy => 700,
            Self::Germany => 900,
            Self::UnitedStates => 1500,
            Self::Other => 2500,
        }
    }

    fn base_tax_percent(self) -> i32 {
        match self {
            Self::Italy => 22,
            Self::Germany => 19,
            Self::UnitedStates => 7,
            Self::Other => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Coupon {
    None,
    Save10,
    VipOnly,
    Bulk,
    FreeShip,
    TaxFree,
    Partner5,
    Other,
}

impl Coupon {
    fn from(value: &str) -> Self {
        match value {
            "" => Self::None,
            "SAVE10" => Self::Save10,
            "VIPONLY" => Self::VipOnly,
            "BULK" => Self::Bulk,
            "FREESHIP" => Self::FreeShip,
            "TAXFREE" => Self::TaxFree,
            "PARTNER5" => Self::Partner5,
            _ => Self::Other,
        }
    }

    fn discount_percent(self, context: &PricingContext) -> i32 {
        match self {
            Self::Save10 if context.subtotal_cents >= 5_000 => 10,
            Self::VipOnly if context.customer_type == CustomerType::Vip => 5,
            Self::Bulk if context.subtotal_cents >= 20_000 => 7,
            Self::Partner5
                if context.customer_type == CustomerType::Partner
                    && context.subtotal_cents >= 12_000 =>
            {
                5
            }
            Self::None
            | Self::Save10
            | Self::VipOnly
            | Self::Bulk
            | Self::FreeShip
            | Self::TaxFree
            | Self::Partner5
            | Self::Other => 0,
        }
    }
}

fn subtotal_after_discount(subtotal_cents: i32, discount_percent: i32) -> i32 {
    subtotal_cents * (100 - discount_percent) / 100
}

fn safe(value: &str) -> String {
    value.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{Order, calculate_total_cents};

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
    fn keeps_regular_customer_example_behavior() {
        let order = order("regular", 10_000, "IT", "", false);

        assert_eq!(12_900, calculate_total_cents(&order));
    }

    #[test]
    fn keeps_premium_coupon_example_behavior() {
        let order = order("premium", 10_000, "DE", "SAVE10", false);

        assert_eq!(10_420, calculate_total_cents(&order));
    }

    #[test]
    fn keeps_vip_coupon_example_behavior() {
        let order = order("vip", 18_000, "IT", "VIPONLY", false);

        assert_eq!(17_980, calculate_total_cents(&order));
    }

    #[test]
    fn keeps_premium_low_subtotal_discount_behavior() {
        let order = order("premium", 9_000, "DE", "", false);

        assert_eq!(11_074, calculate_total_cents(&order));
    }

    #[test]
    fn save10_requires_minimum_subtotal() {
        let order = order("regular", 4_999, "DE", "SAVE10", false);

        assert_eq!(6_848, calculate_total_cents(&order));
    }

    #[test]
    fn free_shipping_coupon_uses_discounted_subtotal_threshold() {
        let order = order("regular", 7_999, "DE", "FREESHIP", false);

        assert_eq!(10_418, calculate_total_cents(&order));
    }

    #[test]
    fn employee_does_not_get_black_friday_discount() {
        let order = order("employee", 10_000, "US", "", true);

        assert_eq!(9_790, calculate_total_cents(&order));
    }

    #[test]
    fn employee_international_fee_is_added_after_free_shipping() {
        let order = order("employee", 12_000, "DE", "FREESHIP", false);

        assert_eq!(10_496, calculate_total_cents(&order));
    }

    #[test]
    fn vip_gets_free_shipping_only_after_discounted_threshold() {
        let order = order("vip", 20_000, "DE", "", false);

        assert_eq!(20_230, calculate_total_cents(&order));
    }

    #[test]
    fn premium_gets_free_shipping_only_after_discounted_threshold() {
        let order = order("premium", 25_000, "DE", "", false);

        assert_eq!(26_775, calculate_total_cents(&order));
    }

    #[test]
    fn taxfree_does_not_apply_in_italy() {
        let order = order("regular", 10_000, "IT", "TAXFREE", false);

        assert_eq!(12_900, calculate_total_cents(&order));
    }

    #[test]
    fn trims_string_inputs_before_pricing() {
        let order = order(" regular ", 10_000, " IT ", "   ", false);

        assert_eq!(12_900, calculate_total_cents(&order));
    }

    #[test]
    fn partner_gets_base_discount_and_partner_coupon() {
        let order = order("partner", 12_000, "DE", "PARTNER5", false);

        assert_eq!(12_752, calculate_total_cents(&order));
    }

    #[test]
    fn partner_coupon_is_ignored_for_other_customers() {
        let order = order("premium", 12_000, "DE", "PARTNER5", false);

        assert_eq!(13_752, calculate_total_cents(&order));
    }

    #[test]
    fn partner_gets_three_percent_black_friday_bonus_instead_of_five() {
        let order = order("partner", 12_000, "DE", "", true);

        assert_eq!(13_038, calculate_total_cents(&order));
    }

    #[test]
    fn partner_gets_free_shipping_at_partner_threshold() {
        let order = order("partner", 18_000, "DE", "", false);

        assert_eq!(18_849, calculate_total_cents(&order));
    }
}
