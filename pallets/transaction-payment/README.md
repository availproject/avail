# Transaction Payment Pallet

This pallet provides the basic logic needed to pay the absolute minimum amount needed for a
transaction to be included. This includes:
  - _weight fee_: A fee proportional to amount of weight a transaction consumes.
  - _length fee_: A fee proportional to the encoded length of the transaction.
  - _tip_: An optional tip. Tip increases the priority of the transaction, giving it a higher
    chance to be included by the transaction queue.

Additionally, this pallet allows one to configure:
  - The mapping between one unit of weight to one unit of fee via [`Config::WeightToFee`].
  - A means of updating the fee for the next block, via defining a multiplier, based on the
    final state of the chain at the end of the previous block. This can be configured via
    [`Config::FeeMultiplierUpdate`]

License: Apache-2.0

## Avail Changes

This fork introduces a new parameter, `length_multiplier`, which dynamically adjusts based on the block length usage of preceding block(s), akin to the existing `weight_multiplier`. This addition provides a more accurate representation of the network's congestion, leading to corresponding adjustments in the inclusion fee of a transaction. Consequently, the formula for calculating the inclusion fee is updated as follows:

```
inclusion_fee = base_fee + (length_multiplier * length_fee) + (weight_multiplier * weight_fee) + tip
```

For further details on the functional changes implemented in this fork, refer to [this commit](https://github.com/availproject/avail/pull/348/commits/6380d0c41d9f5632ee50528fd8192f2ee8476076).
