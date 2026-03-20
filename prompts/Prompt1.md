## Prompt
You inherited a legacy checkout/pricing module. It works, but it is hard to understand, hard to change, and risky to extend.

The code currently handles:

    customer types
    discounts
    shipping
    taxes
    occasional promotions

The implementation is intentionally poor:

    long conditional chains
    duplicated business rules
    mixed responsibilities
    hidden quirks
    low readability
    poor extensibility

Objective

Refactor the pricing module so that:

    current behavior is preserved
    the design becomes easier to extend
    a new customer type and a new promotion can be added safely
    the solution remains simple and understandable

New requirement

Add support for a new customer type: partner

Partner rules:

    base discount: 12%
    free shipping when discounted subtotal is at least 15000 cents
    coupon PARTNER5 adds an extra 5% discount only for partner customers and only when subtotal is at least 12000 cents
    on Black Friday, partner customers get an extra 3% discount instead of the usual 5%

Constraints

    Preserve existing behavior unless the new requirement explicitly changes it
    Do not rewrite everything from scratch
    Do not add new external libraries
    Do not change the input/output contract
    Do not add more conditional complexity to the main legacy flow
    Add a safety net before making structural changes
    Introduce structure only when justified by the problem

Ask for confirmation before any significant changes.
Especially with constraint: "Preserve existing behavior unless the new requirement explicitly changes it". Ask before doing changes.
Create safety net of tests before starting to refactoring and implementing.

## Purpose
Finish kata with maximum usage of agentic AI, while following the rules. I added some extra instructions in the end to be better able to review changes to be made and secure that tests the same tests ran with original code and new code.

## Outcome
Tests run well and the code seems much easier to read.

## Notes
I didn't properly check the created tests before continuing on with AI's implementation. That would have been smart.