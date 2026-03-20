# Prompts

## 1. Preserve behavior first
- **Prompt**: "Before refactoring, read the Rust checkout code, identify the current pricing rules and quirks, and add characterization tests that preserve current behavior. Do not change the public API or business outcomes yet."
- **Purpose**: Build a safety net before structural changes.
- **Outcome**: Added behavior-focused tests for customer discounts, coupons, shipping quirks, taxes, and trimmed string inputs.
- **Notes**: This exposed hidden rules such as the employee international shipping surcharge being applied after free shipping logic.

## 2. Alternatives and tradeoffs
- **Prompt**: "Suggest two or three small refactoring options for the Rust pricing module that improve extensibility without rewriting everything. Compare tradeoffs, especially readability, risk, and how easily a new customer type or coupon can be added."
- **Purpose**: Evaluate design choices before changing the legacy flow.
- **Outcome**: Chose a small rule-oriented refactor using `PricingContext` plus focused helpers for discounts, shipping, and tax instead of a heavier pattern-based rewrite.
- **Notes**: The selected approach kept the flow linear while avoiding one growing conditional block.

## 3. Test-oriented partner change
- **Prompt**: "Add tests for the new `partner` customer type before implementing it. Cover the base discount, the `PARTNER5` coupon rules, Black Friday behavior, and the free shipping threshold."
- **Purpose**: Drive the new requirement with targeted tests.
- **Outcome**: Added tests for partner-specific discounting, coupon eligibility, Black Friday behavior, and free shipping.
- **Notes**: The Black Friday test specifically protects the new 3% rule so it cannot silently regress to the usual 5%.

## 4. Focused implementation prompt
- **Prompt**: "Implement the `partner` customer type in the refactored Rust pricing module without changing the input/output contract and without adding more conditional complexity to the main `calculate_total_cents()` flow."
- **Purpose**: Complete the new requirement while preserving the simplified design.
- **Outcome**: Implemented `partner` support through small helper rules and left the public function signature unchanged.
- **Notes**: The change remained localized to customer and coupon rule helpers.
