# MWVM System Design — Design Documentation

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design, Sourced from mwvm/docs  
**Format**: Aligned with rclob/docs/risk-v2 structure

This directory contains the comprehensive system design documentation for the Morpheum WASM VM (MWVM) ecosystem, synthesized from mwvm/docs sources: bucket-as-service, cost, government, plugin-vm, proposals, securities, test-framework, and nft. The design covers Bucket-as-Service (BaS), hybrid governance, recursive risk controls, deployment costs, extensibility patterns, security, and testing.

## Document Index


| Document                                                       | Description                                                  |
| -------------------------------------------------------------- | ------------------------------------------------------------ |
| [00-mwvm-business-scope.md](00-mwvm-business-scope.md)         | Business scope, core philosophy, locked ownership            |
| [01-overview.md](01-overview.md)                               | Executive summary, key concepts, design principles           |
| [02-architecture.md](02-architecture.md)                       | System architecture, Host API, data flow                     |
| [03-bucket-as-service.md](03-bucket-as-service.md)             | BaS rule set, product types, creation, listing, trading      |
| [04-governance.md](04-governance.md)                           | Hybrid governance, constitutional amendments, MORP proposals |
| [05-recursive-risk.md](05-recursive-risk.md)                   | Depth limiter, skin-in-the-game, effective leverage cap      |
| [06-cost-deployment.md](06-cost-deployment.md)                 | Deployment costs, storage deposits, constitutional params    |
| [07-plugin-vm-extensibility.md](07-plugin-vm-extensibility.md) | Hook vs Pluggable patterns, MIMS, extensibility              |
| [08-securities-testing.md](08-securities-testing.md)           | Security model, agentic testing, overlap penalties           |
| [09-module-structure.md](09-module-structure.md)               | Native vs WASM scope, safe wrappers, integration             |
| [10-scope-boundary.md](10-scope-boundary.md)                   | Scope boundary & responsibility matrix                       |
| [11-mwvm-vs-mormcore-vm.md](11-mwvm-vs-mormcore-vm.md)         | mwvm vs Mormcore VM responsibilities, shared primitives, DRY |


## Quick Reference

- **Core Philosophy**: Host is God, WASM is Pure Compute
- **Governance**: Hybrid — native core (Step 9) + WASM application-level policies
- **BaS Products**: Position-backed, Asset-backed, Mix-backed structural products
- **Recursive Risk**: Max 4 levels, escalating $MORM lock, effective leverage cap ~3.5×
- **Safe Wrappers**: deploy_bucket_product, list_bucket_for_sale, buy_bucket, issue_token, bank_transfer, place_limit_order
- **Constitutional**: All params Step 9 amendable; supermajority ≥66.67%

## Source Documents


| Source                      | Content                                                                          |
| --------------------------- | -------------------------------------------------------------------------------- |
| mwvm/docs/bucket-as-service | BaS rule set, business model, economic view, veMORM, recursive risk              |
| mwvm/docs/government        | design, hypbrid-governance, MORP-GOV-001/2026-02/2026-03, BA-OVERLAP-PENALTY-001 |
| mwvm/docs/cost              | Deployment design, storage deposits                                              |
| mwvm/docs/plugin-vm         | MIMS, Hook vs Pluggable patterns                                                 |
| mwvm/docs/securities        | Security model, overlap features                                                 |
| mwvm/docs/test-framework    | Agentic WASM testing                                                             |
| mwvm/docs/nft               | Bucket-NFT integration                                                           |
| mwvm/docs/proposals         | Version progression, foundational design                                         |


## Related Documentation

- [../bucket-as-service/](../bucket-as-service/) — BaS detailed design
- [../government/](../government/) — Governance proposals
- [../proposals/](../proposals/) — MWVM version progression

