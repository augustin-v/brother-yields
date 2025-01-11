# Nostra Finance (website: nostra.finance)

## Overview
Nostra is a crypto Super App built on Starknet that enables users to lend, borrow, swap and bridge cryptocurrencies through a unified platform. The protocol aims to simplify the crypto experience and onboard the next billion users.

## Key Features
- Non-custodial lending and borrowing
- Multiple pool types ("Volatile", "Stable", "Degen": more risky)
- Flash loans support
- Multiple account creation (up to 255)
- Integration with Chainlink and Pragma oracles
- Prevention of collateral borrowing

## Technical Details
The lending rate is calculated as:
$$ lendingRate_t = borrowRate_t \cdot U_t \cdot (1-generalProtocolFee) $$

The utilization rate is defined as:
$$ U_t = \frac{borrows_t}{deposits_t} $$

## Fees and Tokenomics
- Borrowing rates vary based on utilization
- No fees for flash loans
- Protocol fees taken from borrowing rate
- STRK rewards distribution starting February 22, 2024

## Risks and Considerations
- Smart contract risk
- Oracle risk in degen pools
- Impermanent loss in liquidity pools
- Liquidation risk for unhealthy positions
- Price feed reliability

## Performance Metrics
- Starknet DeFi Spring participant
- 40M STRK distribution over 6-8 months
- Target health factor: 1.5
- Health factor liquidation threshold: 1.0

## Integration with Other Protocols
- Chainlink oracle integration
- Pragma oracle integration
- StableSwap AMM implementation
- CPMM implementation for volatile pools

## Recent Updates
- Selected for Starknet DeFi Spring campaign
- First STRK claims for Pools: March 7, 2024
- First STRK claims for Money Market: March 28, 2024
- DeFi Spring rounds run every two weeks

## Expert Insights for Yield Optimization
- Non-recursive lending supply tracking for rewards
- Recursive grouping of correlated assets
- Strategic pool selection based on asset volatility
- Monitor oracle feeds for optimal trading conditions
