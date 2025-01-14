# TRADING TYPES

Defines convenient types for trading (mostly focused on spread-processing based trading):

    Price
    Amount (base currency quantity)
    Worth = price * amount
    Liq (liquidity) of price, amount and worth as main brick of calculations
    Side from bid/buy or ask/sell
    Depth (order book) as vec of Liq
    Spread in Depth between bids and asks
    OrderToPlace with neccesary data to place limit order
    OrderPlaced with corresponding id etc.


Provides extra funcs:

    depth_util::liqs_l2
    depth_util::drop_worth
    depth_util::worst_execution_price

    See those descriptions in the code


These types are battle-tested as building blocks for type-safe definitions of trading strategies.

So, the first step when received market data is to align with these types and then go on.

There are some trading "enhansers" based on these types which are parts of trading strategies. Probably, the most generic will be open sourced).

