## Substrate modules mocks

This project demostrates how to use mocks with Parity Substrate modules. It contains 'toy' examples and aims to demontrate mocking techniques.

Mocking framework:
- [Mockall](https://docs.rs/mockall/0.6.0/mockall/)

### Motivation

You can use mocks when you want to test modules independently. If module A is dependent on module B and next version of module B is not ready or it contains a bug - mocking can be very useful.

### Description

Project contains two substrate modules and their tests:

- discounts
- complex_prices

**complex_prices** modules depend on **discounts** module in order to calculate price.

### Complex_prices

If you need feature-rich mocks or stateful mocks you can create *mockall* solution, with some additional abstraction layers when use dependent **discounts** module.


Simpler example - how to test the substrate module - you can find [here.](https://substrate.dev/recipes/testing/externalities.html)
