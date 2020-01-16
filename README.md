## Substrate modules mocks

This project demostrates how to use mocks with Parity Substrate modules. It contains 'toy' examples and aims to demontrate mocking techniques.

Mocking frameworks:
- [Mocktopus](https://docs.rs/mocktopus/0.7.5/mocktopus/)
- [Mockall](https://docs.rs/mockall/0.6.0/mockall/)

### Motivation

You can use mocks when you want to test modules independently. If module A is dependent on module B and next version of module B is not ready or it contains a bug - mocking can be very useful.

### Description

Project contains three substrate modules and their tests:

- discounts
- simple_prices
- complex_prices

Both **simple_prices** and **complex_prices** modules depend on **discounts** module in order to calculate price.

### Simple_prices

If you doesn't need complicated mock-scenarios you can use simpler mock solution: *mocktopus* framework conditionally compiled with dependent **discounts** module.

### Complex_prices

If you need feature-rich mocks or stateful mocks you can create combined *moctopus* and *mockall* solution, with some additional abstraction-level when use dependent **discounts** module.
