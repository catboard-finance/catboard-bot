# catboard-bot

Bot for feed things to serve user input, mostly via Discord slash command.

![catboard](https://user-images.githubusercontent.com/97060/151706752-f20104de-48d2-405b-969b-de921aac5869.png)

## Init

```
wrangler login
```

## Setup (done)

```bash
# Product mapping
wrangler kv:namespace create DEVNET_PYTH_PRODUCTS
wrangler kv:namespace create DEVNET_PYTH_PRODUCTS --preview

wrangler kv:namespace create MAINNETBETA_PYTH_PRODUCTS
wrangler kv:namespace create MAINNETBETA_PYTH_PRODUCTS --preview

# Price history
wrangler kv:namespace create DEVNET_PYTH_PRICES
wrangler kv:namespace create DEVNET_PYTH_PRICES --preview

wrangler kv:namespace create MAINNETBETA_PYTH_PRICES
wrangler kv:namespace create MAINNETBETA_PYTH_PRICES --preview
```

## Develop

```bash
# Develop at local
wrangler dev

# Tunnel
cloudflared tunnel login
cloudflared tunnel --url localhost:8787
```

### Preview and Watch

```bash
wrangler preview --watch --url https://$YOUR_WORKER_NAME.workers.dev/api/sync_products
```

## Release

```bash
wrangler publish
```

## TODO

- [ ] Add help+about.

## TOHAVE

- [ ] refactor cluster to use env.
- [ ] Cron to get `SOL`, `ETH`, `BTC` price.
  - [ ] `CHAIN:Crypto.SOL/USD:2022-01-01` = `{low,open,close,average,high}`
- [ ] Save bulk price to kv.
- [ ] Integrate `Swap` with https://docs.jup.ag/
- [ ] Integrate `Stake` with https://github.com/lidofinance/lido-js-sdk
- [ ] Integrate `Lend` with https://docs.francium.io/dev-resource/francium-sdk
- [ ] Add terms, privacy.
