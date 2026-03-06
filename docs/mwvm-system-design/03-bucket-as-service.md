# Bucket-as-Service (BaS) — Rule Set & Design

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/bucket-as-service

## 1. Definitions & Product Types


| Product Type        | Backing Description                                                              | Risk Profile | Typical Use Case                      |
| ------------------- | -------------------------------------------------------------------------------- | ------------ | ------------------------------------- |
| **Position-Backed** | Bucket contains one or more perpetual positions (long/short) with unified margin | Medium-High  | Leveraged strategies, hedging         |
| **Asset-Backed**    | Bucket holds only spot assets (USDC, USDT, USDM, whitelisted tokens) + yield     | Low          | Stable yield products, treasuries     |
| **Mix-Backed**      | Combination of positions + spot assets (e.g., 60% hedged + 40% stable)           | Medium       | Structured products, yield + leverage |


All products are **native buckets** with immutable collateralAssetId and type after creation.

## 2. Creation & Listing Rules

### 2.1 Creation (via `deploy_bucket_product`)

- Must use KYA/VC delegation with claim: `can_deploy_bucket(type, max_value, expiry)`
- Minimum creation deposit: 100 $MORM (refundable on sale or after 90 days)
- Bucket must pass on-chain health snapshot (margin, positions, risk ratio) at creation
- Max 5 new products per DID per epoch (constitutional, amendable)
- Governance whitelist for allowed collateralAssetId types

### 2.2 Listing for Sale (via `list_bucket_for_sale`)

- Must attach immutable health snapshot + metadata (backing description, risk summary)
- Listing fee: 5 $MORM (burn 50%, insurance 30%, treasury 20%)
- Seller must lock bucket (transfer to escrow) during listing
- Minimum listing duration: 24 hours (prevents flash-sale rug pulls)
- Price can be in $MORM, stable, or whitelisted asset (settled atomically)

### 2.3 Visibility & Discovery

- All listed products in public secondary market index
- Reputation score of seller displayed
- Optional "Verified Issuer" badge for high-reputation agents (governance tunable)

## 3. Trading & Settlement Rules

### 3.1 P2P Purchase (`buy_bucket`)

- Buyer must provide KYA/VC with claim `can_buy_bucket(listing_id, max_price)`
- **Atomic escrow**:
  1. Buyer payment locked
  2. Bucket health re-verified (snapshot comparison)
  3. Bucket transferred + payment released (or full refund on failure)
- **Settlement is atomic** — buyer receives clean bucket or money back
- No partial ownership — full bucket transfer only

### 3.2 Post-Sale Obligations

- Seller cannot modify sold bucket (immutable transfer)
- Buyer inherits all positions and margin exactly as listed
- Hidden risk discovered post-sale triggers insurance claim

### 3.3 Secondary Market Trading

- Buyers can re-list (new health snapshot required)
- Each resale pays 2% fee in $MORM (same split as listing)
- Chain tracks full ownership history (immutable log)

## 4. Security & Exploit-Aware Countermeasures

### 4.1 Mandatory Protections

- All actions require valid VC with scoped claims
- On-chain health snapshot at listing and purchase
- Atomic escrow for every sale (no reentrancy window)
- Per-DID creation/listing quotas + daily value caps (constitutional)

### 4.2 Economic Penalties


| Misconduct                          | Penalty                                                       | Impact on $MORM   |
| ----------------------------------- | ------------------------------------------------------------- | ----------------- |
| **Misrepresentation** (hidden risk) | 100% deposit slash + 30-day reputation ban + insurance payout | Burned + treasury |
| **Rug Pull / Drain**                | Full slashing + permanent DID blacklist                       | Burned            |
| **Spam Creation**                   | Deposit burn + temporary quota reduction                      | Burned            |
| **Oracle Manipulation**             | Oracle provider + seller joint slashing                       | Burned            |


### 4.3 Insurance Fund

- Funded by 30% of creation/listing/sale fees (in $MORM)
- Pays verified victims up to listing-time snapshot value
- Remainder used for $MORM buybacks/burns

### 4.4 Reputation System

- Successful sales increase seller reputation
- Proven misconduct decreases it
- High-reputation agents get higher quotas, lower fees, "Verified" badge

## 5. Economic Model for $MORM Value Appreciation

1. **Fee Demand** — Creation, listing, resale fees paid in $MORM → direct demand
2. **Staking & Yield** — Stake $MORM for higher quotas, verified badge, fee discounts
3. **Burn Mechanism** — 40–50% of fees burned → deflationary pressure
4. **Treasury & Governance** — 20–30% to treasury → buybacks, development, liquidity mining
5. **Liquidity Mining** — Provide liquidity to secondary bucket market → earn $MORM rewards

## 6. veMORM Integration

- **veMORM balance** = lock proof for skin-in-the-game (recursive risk)
- Longer/higher locks give more buffer capacity per $MORM
- Required lock % met via veMORM (checked at deposit/leverage time)
- Tree-first-loss absorption draws from pro-rata veMORM balances

## 7. NFT Integration

- Buckets are natively NFT-compatible (unique ID, owner, transfer, metadata slot)
- Rich NFT features (metadata, royalties, fractionalization) optional via WASM
- `deploy_bucket_product` can mint bucket as NFT with metadata
- `list_bucket_for_sale` / `buy_bucket` work like NFT marketplace

## 8. Governance & Upgradability

- All parameters (quotas, fees, insurance split, min deposit, reputation thresholds) are **constitutional** (Step 9)
- Agents can propose new bucket templates or collateralAssetId support via governance
- Emergency pause (Safe Mode) can be activated by supermajority during attacks

## Related Documents

- [05-recursive-risk.md](05-recursive-risk.md) — Depth limiter, skin-in-the-game
- [04-governance.md](04-governance.md) — Constitutional params, MORP-GOV-001
- [../bucket-as-service/](../bucket-as-service/) — Full BaS source docs

