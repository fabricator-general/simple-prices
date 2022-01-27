# Simple Terminal Price Feed

Get the live price and 24h % change of a few cryptoassets in your terminal
<br></br>

Build release
```
cargo build --release
```
Run with comma delimited asset symbols  
Currently only `[btc, eth, sol, luna, avax]` are available
```
cargo run --release btc,sol,luna
```
<br></br>

**Note:** New terminal price output does not replace old terminal output, this is not an intended feature and is a todo. Any help is appreciated. 