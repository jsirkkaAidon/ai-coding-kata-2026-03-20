# Solution

## What was wrong in the legacy design
- `calculate_total_cents()` mixed normalization, discount rules, shipping rules, tax rules, and total calculation in one long function.
- Customer, coupon, shipping, and tax logic were all expressed as long conditional chains.
- Extending behavior required editing the same large block in multiple places, which increased the chance of regressions.
- Hidden quirks were hard to see, such as free shipping being overridden by the employee international surcharge and shipping thresholds being evaluated on the discounted subtotal.

## What changed
- Added characterization tests first to preserve existing behavior before restructuring the code.
- Introduced a small `PricingContext` to normalize and hold the parsed order data.
- Split the pricing logic into focused helpers:
  - `DiscountRules`
  - `ShippingRules`
  - `TaxRules`
- Replaced string-based conditional chains inside the main flow with small enums for `CustomerType`, `Country`, and `Coupon`.
- Added the new `partner` customer type and `PARTNER5` coupon behavior.

## Why the new structure is easier to extend
- The main `calculate_total_cents()` flow is now linear and easy to explain: build context, compute discount, compute shipping, compute tax, compute total.
- Customer-specific behavior is concentrated in `CustomerType` methods instead of scattered across the function.
- Coupon-specific behavior is concentrated in `Coupon` methods, so adding a new coupon no longer requires editing unrelated shipping or tax logic.
- Shipping and tax rules are separated from discount rules, which makes each change smaller and easier to review.
- The test suite now protects both legacy behavior and the new partner rules.

## Rejected AI suggestion
- **Suggestion rejected**: Replace the module with a full strategy-pattern or trait-object design for every rule type.
- **Why it was rejected**: That approach would have introduced more moving parts than the kata needed and would have felt too much like a rewrite. A smaller refactor with focused helpers achieved the extensibility goal with lower risk and better readability.
