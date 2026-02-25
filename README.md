#  Black-Scholes Quant Terminal (Rust)

A high-performance financial terminal built in Rust for pricing European options and analyzing real-time risk metrics (Greeks). This project demonstrates the integration of asynchronous systems, statistical computing, and reactive graphical interfaces.

##  Key Features
- **Reactive Calculation:** Instant recalculation of Call/Put prices and Greeks (Delta, Gamma, Theta, Vega) upon any parameter change.
- **Real-Time Market Data:** Integration with Yahoo Finance API to fetch B3 (Brazilian Stock Exchange) asset prices and the Selic (interest) rate.
- **Automated Volatility:** Statistical calculation of annualized historical volatility based on the last 6 months of trading data.
- **Async Engine:** Fluid user interface guaranteed by `Tokio` runtime and communication via `MPSC Channels`.

##  Tech Stack
- **Language:** [Rust](https://www.rust-lang.org/)
- **GUI:** [egui/eframe](https://github.com/emilk/egui)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Financial Math:** [statrs](https://crates.io/crates/statrs)
- **Market Data:** [yahoo_finance_api](https://crates.io/crates/yahoo_finance_api)

##  The Mathematical Model
The terminal implements the Black-Scholes-Merton model with adjustments for the Brazilian market:
- **Interest Rates:** Automatic conversion of the Selic rate to log-continuous interest rates.
- **Time Horizon:** Calculation based on 252 business days (B3 standard).
- **The Greeks:** Analytical calculation of first and second-order derivatives for professional risk management.



##  How to Run
1. Ensure you have the Rust toolchain installed (`rustup`).
2. Clone the repository:
   git clone [https://github.com/YOUR_USERNAME/black-scholes-rust.git](https://github.com/YOUR_USERNAME/black-scholes-rust.git)
3. Run the project: cargo run --release


Roadmap
[ ] Implementation of Implied Volatility calculation (Newton-Raphson Method).

[ ] Interactive Payoff Charts.

[ ] Support for multiple assets in a simulated portfolio.