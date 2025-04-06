# VRF GenKit

A standalone VRF (Verifiable Random Function) Prover accessible locally through CLI or as an autonomous on-chain prover.

## Features

- Generate & Verify VRF proofs using Elliptic Curve Cryptography (ECC)
- Listen for on-chain VRF requests and automatically respond
- Can be integrated within Foundry projects (using ffi, parseJson cheats)
- JSON output for proofs
- Supports SECP256K1_SHA256_TAI scheme (SECP256K1 with SHA256)

> **Supported Curves**: Only SECP256K1 elliptic curve is implemented currently. Support for additional curves may be added in future versions.
>
> **Draft Status**: This program (via the libraries it depends on, implement a non-finalized draft of RFC 9381 termed  "draft-irtf-cfrg-vrf-04", VRF Draft 04)

## Installation

### As Foundry Dependency

1. Add repository as dependency
```bash
forge install https://github.com/nomad-lw/vrf-gen
```
2. Build
```bash
cd lib/vrf-gen
cargo build
```
3. The CLI binary is now available at `lib/vrf-gen/target/debug/gen_vrf`

### As Standalone

1. TODO

## Configuration

Add the following variables to your `.env` file (or create one):

Note: If you do not provide a secret key (VRF_SECRET_KEY), a new one will be generated and automatically saved to `.env`.

```env
# REQUIRED FOR USE AS FOUNDRY DEPENDENCY
VRF_SECRET_KEY="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef" # A 32 byte hex string

# FOR ONCHAIN SUBMISSION OF VRF PROOFS
VRF_PROVER_ADDR="0x1111111111111111111111111111111111111111" # An Ethereum-compatible EOA address
VRF_PROVER_PRIVATE_KEY="0x2222222222222222222222222222222222222222222222222222222222222222" # EOA's private key
```


## Usage

### CLI

```
Usage: gen_vrf [OPTIONS] --operation <OPERATION>

Options:
  -o, --operation <OPERATION>
          Operation to perform: "prove", "verify", or "listen"
  -m, --message <MESSAGE>
          Message to use for the operation (hex-string)
  -p, --pi <PI>
          Proof (pi) value to use for the operation (hex-string) (only if operation is "verify")
  -f, --force-key-gen
          Force generation of a new secret key
  -s, --silent
          Silent mode
      --soft
          Soft mode
  -j, --json <JSON>
          JSON output file suffix [default: ]
      --rpc-url <RPC_URL>
          Ethereum RPC URL (only for listen mode) [default: ws://localhost:8545]
      --contract-address <CONTRACT_ADDRESS>
          Contract address to listen to (only for listen mode) [default: 0xYourContractAddressHere]
  -h, --help
          Print help
  -V, --version
          Print version

```

#### Examples

##### Generate a VRF Proof
```bash
gen_vrf --operation prove --message <hex_message> [--force-key-gen] [--silent] [--soft] [--json <suffix>]

# Example:
gen_vrf --operation prove --message 0x1234 --json _test
```

##### Verify a VRF Proof
```bash
gen_vrf --operation verify --message <hex_message> --pi <hex_encoded_proof> [--force-key-gen] [--silent]

# Example:
gen_vrf --operation verify --message 0x1234 --pi 0xabcd...
```

##### On-chain Listener

> TODO


### Within Foundry

#### Examples

NOTE: These examples make use of external libraries:
- [Solady](https://github.com/vectorized/solady)
- [VRF Solidity](https://github.com/nomad-lw/vrf-solidity)

##### Generate and Retrieve a VRF Proof

```solidity
// SPDX-License-Identifier: AGPL-3.0-or-later
pragma solidity ^0.8.26;

import {VRF} from "vrf-solidity/contracts/VRF.sol";
import {Test, console} from "forge-std/Test.sol";
import {LibString as Strings} from "solady/utils/LibString.sol";

contract VRFTest is Test {
    struct VrfDataIntermediate {
        bytes32 hash;
        bytes32 message;
        bytes proof;
        bytes public_key;
        uint256 secret_key;
    }

    struct VrfData {
        bytes32 hash;
        uint256 message;
        bytes proof;
        bytes public_key;
        uint256 secret_key;
    }

    function generate_vrf_proof(bytes32 alpha) public {
        string[] memory inputs = new string[](8);
        string memory encoded_alpha = Strings.toHexString(uint256(alpha), 32);
        string memory output_file_suffix = vm.toString(vm.unixTime());
        inputs[0] = "lib/vrf-gen/target/debug/gen_vrf";
        inputs[1] = "-o";
        inputs[2] = "prove";
        inputs[3] = "-m";
        inputs[4] = encoded_alpha;
        inputs[5] = "--silent";
        inputs[6] = "--json";
        inputs[7] = output_file_suffix;
        console.log("Encoded alpha:", encoded_alpha);

        bytes memory res = vm.ffi(inputs);
        console.log("res:", string(res));
        assertEq(string(res), "Ok");

        // retrieve proof
        VrfData memory vrf_data = load_vrf_proof(output_file_suffix);
        assertEq(bytes32(vrf_data.message), alpha);
    }

    function load_vrf_proof(string memory output_file_suffix) public returns (VrfData memory vrf_data) {
        string memory ROOT = vm.projectRoot();
        string memory PATH = string.concat(ROOT, "/vrf_proof", output_file_suffix, ".json");
        string memory json = vm.readFile(PATH);
        console.log("Raw Data:", json);
        vrf_data = parse_vrf_json(json);
    }

    function parse_vrf_json(string memory json) internal pure returns (VrfData memory vrf_data) {
        bytes memory data = vm.parseJson(json);
        // parseJson/abi.decode does not like coercing dynamic bytes into non-byte types
        VrfDataIntermediate memory vrf_data_i = abi.decode(data, (VrfDataIntermediate));
        vrf_data.hash = vrf_data_i.hash;
        vrf_data.message = uint256(vrf_data_i.message);
        vrf_data.proof = vrf_data_i.proof;
        vrf_data.public_key = vrf_data_i.public_key;
        vrf_data.secret_key = vrf_data_i.secret_key;
    }
}
```


### Flags

- `--force-key-gen`: Generate a new secret key regardless of existing one
- `--silent`: Minimal output (useful for scripting)
- `--soft`: Output JSON to stdout instead of file
- `--json <suffix>`: Custom suffix for JSON output file, formatted as "vrf_proof<suffix>.json"


## Security Considerations

- Your secret key should be kept secure.. duh.
- Environment variables are preferred over command-line arguments for sensitive data
- **Experiment Warning**: This is experimental software. Use at your own risk.
- **No Guarantees**: No warranties or guarantees of functionality, security, or correctness are provided.
- **Unaudited**: This software has not undergone formal security audits.
- **No Liability**: The authors and contributors accept no liability for any damages or losses resulting from the use of this software.


## License

See [LICENSE.md](LICENSE.md) for details.
Licensed under the [GNU Affero General Public License v3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) (AGPL-3.0)
