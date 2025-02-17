# Spearmint

Spearmint is Automated Trading Tool.

This console application is build to automate my trading on Binance using a Martingale-like strategy.

The bot creates and executes orders, DCA-ing based on predefined conditions or indicators.

```bash
./spearmint run-all --duration 30


+--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------+
| MFI    | Pair    | Price      | AVG         | P.Change | T.Price     | B.Price    | B.MFI  | MFI Dir | Wallet | Cycle | M.Position |
+====================================================================================================================================+
| 0.0000 | BTCUSDT | 96191.3500 | 103738.6500 | -7.28%   | 103738.6500 | 96191.3500 | 0.0000 | DOWN    | 9.2886 | 1     | 0          |
|--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
| 0.0000 | ETHUSDT | 2696.7200  | 0.0000      | 0.00%    | 3421.3900   | 2696.7200  | 0.0000 | DOWN    | 9.2886 | 2     | 0          |
|--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
| 0.0000 | SOLUSDT | 182.5900   | 261.2400    | 0.11%    | 261.2400    | 182.5900   | 0.0000 | DOWN    | 9.2886 | 2     | 0          |
|--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
| 0.0000 | SUIUSDT | 3.2665     | 0.0000      | 0.00%    | 4.4593      | 3.2665     | 0.0000 | DOWN    | 9.2886 | 1     | 0          |
+--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------+


```

## Quick Start Guide

### Installation

1. Clone this repository: `git clone <REPO_URL>`
2. Install dependencies and build executable:

   ```bash
   cargo build --release
   ```

   It will build 2 executable files

3. Go to `./target/release/spearmint` to get executable files `spearmint` and `ticker`

## Usage

There are two types of configuration files: one for account binding and the other for bot configuration. You can create them based on the examples in the config folder.

### New Exchange Account Binding

Create account binding file. Example `configs/config.bind.example.toml`

```toml
kind = "bind"
api_key = ""
secret_key = ""
platform = "binance"

```

execute `apply` command to save it to storage,

```bash
./spearmint apply --file ./configs/config.bind.example.toml
```

### New Trading Bot

Create account binding file. Example `configs/bot.dogeusdt.toml`

```toml
kind = "bot"
title = "DOGEUSDT"
pair = "DOGEUSDT"
base = "DOGE"
quote = "USDT"
platform = "binance"
strategy = "helldiver"
status = "PAUSED"

[parameters]
cycle = "repeat"
first_buy_in = 10.0

[parameters.entry]
mfi_below = 25.0
mfi_callback = 10.0
price_change_below = -1.0
price_callback = 0.5
amount_ratio = 1

[parameters.take_profit]
price_change_above = 1.2
price_callback = -0.3

[[margin.margin_configuration]]
mfi_below = 25.0
mfi_callback = 10.0
price_change_below = -2.0
price_callback = 0.5
amount_ratio = 4

[[margin.margin_configuration]]
mfi_below = 25.0
mfi_callback = 10.0
price_change_below = -7.0
price_callback = 0.5
amount_ratio = 15


```

execute `apply` command to save it to storage,

```
./spearmint apply --file ./configs/bot.dogeusdt.toml
```

### _spearmint_ Commands

1. Check running channel status

   ```
   cargo run -- status --name BTC_USDT_BINANCE
   ```

2. Check running bots

   ```bash
   ./spearmint list


   +----------+----------+----------+-----------+--------+--------+
   | Title    | Pair     | Platform | Strategy  | Cycle  | Status |
   +==============================================================+
   | DOGEUSDT | DOGEUSDT | binance  | helldiver | repeat | PAUSED |
   |----------+----------+----------+-----------+--------+--------|
   | BTCUSDT  | BTCUSDT  | binance  | helldiver | repeat | ACTIVE |
   |----------+----------+----------+-----------+--------+--------|
   | ETHUSDT  | ETHUSDT  | binance  | helldiver | repeat | ACTIVE |
   +----------+----------+----------+-----------+--------+--------+
   ```

3. Check my assets in exchange

   ```bash
   ./spearmint account --platform binance


   +--------+-------------+
   | Asset  | Free        |
   +======================+
   | BTC    | 0.000       |
   |--------+-------------|
   | ETH    | 0.000       |
   |--------+-------------|
   | BNB    | 0.000       |
   |--------+-------------|
   ```

4. Run all bots

   ```bash
   ./spearmint run-all --duration 30


   +--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------+
   | MFI    | Pair    | Price      | AVG         | P.Change | T.Price     | B.Price    | B.MFI  | MFI Dir | Wallet | Cycle | M.Position |
   +====================================================================================================================================+
   | 0.0000 | BTCUSDT | 96191.3500 | 103738.6500 | -7.28%   | 103738.6500 | 96191.3500 | 0.0000 | DOWN    | 9.2886 | 1     | 0          |
   |--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
   | 0.0000 | ETHUSDT | 2696.7200  | 0.0000      | 0.00%    | 3421.3900   | 2696.7200  | 0.0000 | DOWN    | 9.2886 | 2     | 0          |
   |--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
   | 0.0000 | SOLUSDT | 182.5900   | 261.2400    | 0.11%    | 261.2400    | 182.5900   | 0.0000 | DOWN    | 9.2886 | 2     | 0          |
   |--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------|
   | 0.0000 | SUIUSDT | 3.2665     | 0.0000      | 0.00%    | 4.4593      | 3.2665     | 0.0000 | DOWN    | 9.2886 | 1     | 0          |
   +--------+---------+------------+-------------+----------+-------------+------------+--------+---------+--------+-------+------------+


   ```

### _ticker_ Commands

Use this command to listen price feed and calculate MFI indicator

1m ticker

```
./ticker --kline 1m --path ticker1m.db
```

1d ticker

```
./ticker --kline 1d --path ticker1d.db
```
