# Solution

## What Was Wrong in the Legacy Design

The original checkout module worked, but it was fragile and hard to evolve:

- A long, single function mixed discount, shipping, tax, and promotion logic.
- Business rules were encoded in large conditional chains, which increased change risk.
- Rules for customer types and coupons were tightly coupled, so one change could affect unrelated behavior.
- Important quirks (for example, Black Friday behavior, shipping overrides, and tax exceptions) were hidden inside procedural flow rather than named rule units.
- There was no safety net before refactoring, so behavior-preserving changes were risky.

In this kata, those issues were visible in [rust-kata/src/lib.rs](rust-kata/src/lib.rs) before refactoring.

## What Changed

### 1. Safety Net First

Before structural refactoring, characterization tests were added to lock existing behavior, including edge cases and known examples.

- Tests now live in [rust-kata/src/tests.rs](rust-kata/src/tests.rs).
- They cover existing customer and coupon behavior, shipping/tax quirks, and Black Friday effects.

### 2. Internal Refactor Without API Contract Changes

The public contract was preserved:

- `Order` shape is unchanged.
- `calculate_total_cents(&Order) -> i32` is unchanged.

The internals were split into focused helpers in [rust-kata/src/lib.rs](rust-kata/src/lib.rs):

- `base_discount_percent`
- `coupon_discount_percent`
- `black_friday_discount_percent`
- `shipping_cents`
- `base_shipping_cents`
- `tax_percent`

This kept the main flow simple while preserving legacy behavior.

### 3. New Requirement Implemented: `partner`

Added support for the new customer type and promotion rules:

- Base discount: 12%
- Free shipping when discounted subtotal >= 15000
- `PARTNER5` coupon: +5% only for partner and only when subtotal >= 12000
- On Black Friday, partner gets +3% (instead of the usual +5%)

These are implemented in [rust-kata/src/lib.rs](rust-kata/src/lib.rs) and verified by tests in [rust-kata/src/tests.rs](rust-kata/src/tests.rs).

## Why the New Structure Is Easier to Extend

The updated structure improves extensibility because:

- Rules are grouped by responsibility (discount/coupon/Black Friday/shipping/tax) rather than embedded in one giant flow.
- Adding a new customer type or coupon generally means editing one focused rule function, not many scattered condition blocks.
- Existing behavior is protected by tests, so refactoring and feature additions are safer.
- The main calculation function now reads as orchestration, making intent and execution order easier to understand.

This design is still simple (functions, not framework-heavy patterns), but materially safer to modify.

## One AI Suggestion Rejected and Why

### Rejected suggestion

"Rewrite the pricing engine from scratch using a full Strategy/Factory hierarchy with one class/trait object per rule."

### Why it was rejected

- It violated the kata intent to avoid rewriting everything.
- It introduced unnecessary abstraction for the current scope.
- It would have increased migration risk and made behavior parity harder to prove quickly.
- A smaller, function-level refactor achieved clarity and extensibility while preserving behavior and contract.
