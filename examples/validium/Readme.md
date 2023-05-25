## Data Attestation

This repository contains examples on how to verify data availability on the Ethereum network.
More details can be found in the official documentation
for [Validiums](https://availproject.github.io/using-avail/validiums).

Example of submitting data and dispatching data root can be found in [submitData.js](submitData.js).
This example will submit data transaction to the Avail and try to dispatch data root
which will be bridged via Nomad optimistic bridge to the Ethereum network.
Nomad bridge is a type of optimistic bridge, so it is necessary to wait for little more than 30 minutes
in order bridge data root to the Ethereum network.

Example for verifying data availability on the Ethereum network can be found in [submitProof.js](submitProof.js).
This example queries the Avail for the Merkle proof which is then checked against deployed
contract [ValidiumContract.sol](contracts%2FValidiumContract.sol).

In order to run these examples, install all necessary dependencies via **npm** 
and make sure that all variables in **.env** file are populated (`DATA`, `SURI`, etc...), 
more details in the Avail documentation for [Validiums](https://availproject.github.io/using-avail/validiums#verify-data-availability-on-ethereum).

Running submit data example:
```bash
  node submitData.js
```

Checking data availability on Ethereum network:
```bash
  node submitProof.js
```

