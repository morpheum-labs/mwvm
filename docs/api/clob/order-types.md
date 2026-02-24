In trading systems (especially in centralized exchanges like Binance, Coinbase, NASDAQ, or crypto exchanges), the **matching engine** is the core software that matches buy and sell orders according to specific rules. Different **order types** tell the matching engine *how* to handle your order — when it can be matched, at what price, for how long, etc.

Here’s a clear breakdown of the most common order types and how the matching engine treats each one:

| Order Type              | What It Means                                                                 | How the Matching Engine Handles It                                                                                  | Typical Use Case                          |
|-------------------------|-------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|-------------------------------------------|
| **Market Order**        | “Buy or sell immediately at the best available price right now.”             | Instantly matches with the best opposite orders in the order book (no price specified). Takes liquidity.            | When you need execution NOW, don’t care about price slippage. |
| **Limit Order**         | “Buy or sell only at this price (or better).”                                 | Placed in the order book. Only matches if market price reaches your limit price or better (lower for buy, higher for sell). Adds liquidity. | Precise entry/exit price control. Most common order type. |
| **Stop Order** (Stop-Market) | “When price hits this level, place a market order.”                          | Triggers a market order when the stop price is touched. Becomes a market order once triggered.                      | Basic stop-loss or breakout entries.      |
| **Stop-Limit Order**    | “When price hits stop price, place a limit order at this limit price.”       | Two prices: stop price (trigger) + limit price. Once triggered, becomes a limit order (may not fill if price moves past limit). | Stop-loss with price protection (no slippage beyond limit). |
| **Take-Profit Order**   | Usually just a regular limit order on the opposite side, or a stop-limit.    | Same as limit or stop-limit, depending on exchange implementation.                                                 | Locking in profits automatically.         |
| **Iceberg Order**       | “Only show a small part of my large order.”                                   | Displays only a visible portion (e.g., 100 shares). When that fills, automatically reveals the next slice. Hidden part stays hidden. | Large traders/institutions who don’t want to scare the market. |
| **Post-Only Order**     | “Only add liquidity — cancel if it would take liquidity immediately.”        | If the limit price would cross the spread and match immediately, the order is canceled or adjusted. Guarantees maker fee (or rebate). | Makers who want fee rebates, algo traders. |
| **Fill-or-Kill (FOK)**  | “Fill the entire order immediately or cancel it all.”                        | Must be fully matched right now against resting orders. If even 1 unit can’t be filled, whole order is canceled.    | Large traders who don’t want partial fills and signaling. |
| **Immediate-or-Cancel (IOC)** | “Fill whatever you can right now, cancel the rest.”                        | Partially fills instantly with whatever is available, cancels unfilled portion. No resting in book.                 | Getting as much as possible instantly without leaving an order. |
| **All-or-None (AON)**   | “Only fill if the entire order can be filled (can rest in book).”            | Similar to FOK but can wait in the order book until the full size is available.                                    | Rarely used because it can sit forever.   |
| **Good-Til-Canceled (GTC)** | Stays active until fully filled or you manually cancel.                    | Default on most exchanges. Rests in book indefinitely.                                                              | Normal long-term limit orders.            |
| **Good-Til-Date (GTD)** | Active until a specific date/time you set.                                    | Automatically canceled at the specified expiration.                                                                 | Orders that are only valid for a certain period. |
| **Day Order**           | Automatically canceled at the end of the trading day/session.                 | Common in traditional stock markets.                                                                                | Intraday traders.                         |
| **OCO (One-Cancels-the-Other)** | Pair of orders: if one executes, the other is automatically canceled.     | Usually a limit + stop-limit pair (take-profit + stop-loss). The matching engine monitors both; fills one → cancels the other. | Classic bracket order for bucket management. |
| **Trailing Stop**       | Stop price “trails” the market price by a fixed distance or percentage.       | The stop price dynamically adjusts in favorable direction. Triggers a market or stop-limit when trailed stop is hit. | Locking in profits during strong trends without knowing the top/bottom. |

### Price-Time Priority (How the Engine Actually Matches)
Almost all matching engines use **Price-Time Priority** (sometimes called FIFO – First In, First Out at the same price):

1. **Best price first**  
   – Buys: highest bid price gets filled first  
   – Sells: lowest ask price gets filled first

2. **At the same price → earliest order first**  
   That’s why you sometimes see “time priority” matter when many people place limit orders at the exact same price.

Some exchanges (especially some crypto ones) offer **Pro-rata** or **Price-Time with size weighting**, but price-time is by far the most common.

### Maker vs Taker (Important for Fees)
- **Maker** = adds liquidity (limit orders that rest in the book) → usually lower fees or rebates
- **Taker** = removes liquidity (market orders, or aggressive limit orders that cross the spread) → higher fees

Order types like Post-Only help you guarantee maker status.

Let me know which exchange/platform you’re interested in (Binance, Bybit, Kraken, Interactive Brokers, etc.) and I can tell you exactly which of these they support and any special names or twists they have!