import backend_keys

let seed: [UInt8] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

let unifiedSpendingKeyBytes: [UInt8] = [180, 208, 214, 194, 3, 32, 166, 3, 186, 151, 20, 139, 99, 33, 212, 134, 101, 192, 119, 208, 167, 21, 119, 228, 7, 152, 74, 140, 84, 209, 236, 235, 53, 57, 109, 65, 44, 178, 2, 169, 3, 240, 68, 53, 97, 0, 0, 0, 128, 234, 206, 224, 230, 180, 69, 172, 115, 57, 184, 221, 212, 204, 73, 161, 165, 210, 199, 46, 10, 200, 142, 73, 167, 9, 104, 89, 58, 200, 121, 136, 69, 228, 111, 114, 225, 87, 227, 210, 233, 213, 86, 13, 107, 118, 27, 114, 52, 191, 0, 154, 130, 192, 9, 11, 6, 220, 168, 246, 77, 183, 221, 52, 14, 198, 139, 75, 203, 159, 201, 17, 117, 90, 15, 68, 49, 79, 15, 95, 118, 205, 210, 120, 134, 40, 80, 122, 89, 82, 180, 159, 230, 35, 232, 105, 9, 144, 208, 234, 146, 137, 215, 60, 50, 183, 254, 149, 253, 137, 42, 232, 60, 251, 179, 135, 99, 159, 238, 119, 130, 4, 75, 67, 113, 67, 10, 191, 0, 1, 188, 15, 115, 131, 15, 198, 187, 156, 71, 47, 94, 152, 220, 110, 161, 168, 50, 123, 203, 190, 135, 6, 11, 110, 180, 235, 248, 125, 93, 89, 193, 0, 64, 207, 174, 34, 199, 252, 227, 180, 123, 230, 0, 80, 146, 93, 219, 173, 12, 108, 17, 103, 102, 144, 226, 101, 143, 138, 175, 53, 52, 12, 34, 185, 103, 172, 47, 218, 133, 127, 111, 155, 112, 212, 44, 34, 186, 124, 174, 31, 169, 99, 123, 229, 175, 141, 181, 225, 250, 107, 144, 184, 248, 64, 255, 239, 143]

let unifiedSpendingKey = try ZcashUnifiedSpendingKey.fromSeed(
    params: ZcashConsensusParameters.mainNetwork,
    seed: seed,
    accountId: ZcashAccountId(id: 0)
);

let encodedUnifiedFullViewingKey = "uview1ac6swpuurz2cgr8ktk630exjrz45fsuc4jeqwgg4dm33stl8awhcju0kyaxvw58405jla4k7rqfcw35l4rsj3ta74a2me8p9hh52uxp5zm5wk60pkpy7242wdhdgm265ah3pjqe03m0vax0wa2k4yqnu0gzmnnkt2sjmxeg7s3v8j55mnrzwqznttkaj86ghs2hzp0pstlvw4zlc7kqc2n98h6xluat24829f5fvgue0w8m9r2fwtyzrdvxf7vwu67fd0wdtc0m3m952prz3w7sc8s42v48u9nsd4gld2pgjfzu9qxxxs06mdtkz2dcda0926wulk0t564k3gs6mjm04qmj6e2yrj8vmjh3flh6fg7y4k5fjj09xmv2ffv6ua7e97fszgfpp94uytsq0cu35dd53n45ua4m43gha3dquw60as4xrynllveyjczyffsd8fm6npe88pmg6j6kpjfapuurnrwjya3gz2xfvmyv5r433rsnkra8h"

let sapling_diversifier = try ZcashDiversifier(bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])

let saplingIvkPaymentAddressBytes: [UInt8] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 150, 127, 9, 126, 127, 135, 242, 241, 100, 51, 242, 226, 238, 170, 123, 25, 163, 69, 216, 183, 101, 10, 82, 150, 119, 1, 188, 11, 103, 156, 95]

let orchard_diversifier = try ZcashOrchardDiversifier.fromBytes(bytes: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])

let orchardIvkPaymentAddressBytes: [UInt8] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 188, 133, 2, 30, 187, 222, 192, 24, 118, 136, 143, 249, 3, 127, 83, 48, 137, 67, 228, 146, 86, 27, 251, 163, 42, 159, 247, 98, 150, 25, 7]

let accountPrivKeyBytes: [UInt8] = [207, 174, 34, 199, 252, 227, 180, 123, 230, 0, 80, 146, 93, 219, 173, 12, 108, 17, 103, 102, 144, 226, 101, 143, 138, 175, 53, 52, 12, 34, 185, 103, 172, 47, 218, 133, 127, 111, 155, 112, 212, 44, 34, 186, 124, 174, 31, 169, 99, 123, 229, 175, 141, 181, 225, 250, 107, 144, 184, 248, 64, 255, 239, 143]

let accountPrivKey = try ZcashAccountPrivKey.fromSeed(
    params: ZcashConsensusParameters.mainNetwork,
    seed: seed,
    accountId: ZcashAccountId(id: 0)
);

// Test ZcashUnifiedSpendingKey.fromSeed
assert(unifiedSpendingKey.toBytes(era: ZcashKeysEra.orchard) == unifiedSpendingKeyBytes)

// Test ZcashUnifiedFullViewingKey.encode
assert(unifiedSpendingKey.toUnifiedFullViewingKey()
    .encode(params: ZcashConsensusParameters.mainNetwork) == encodedUnifiedFullViewingKey)

// Test ZcashSaplingIvk.toPaymentAddress
assert(unifiedSpendingKey.toUnifiedFullViewingKey()
    .sapling()!.toIvk(scope: ZcashScope.external)
    .toPaymentAddress(diversifier: sapling_diversifier)!.toBytes() == saplingIvkPaymentAddressBytes)

// Test ZcashOrchardIvk.toPaymentAddress
assert(unifiedSpendingKey.toUnifiedFullViewingKey()
    .orchard()!.toIvk(scope: ZcashOrchardScope.external)
    .address(diversifier: orchard_diversifier).toRawAddressBytes() == orchardIvkPaymentAddressBytes)

// Test ZcashAccountPrivKey.fromSeed
assert(accountPrivKey.toBytes() == accountPrivKeyBytes)

// Test ZcashExtendedPrivKey.withSeed, ending up in a private key.
let accountExtendedPrivKey = try ZcashExtendedPrivKey.withSeed(data: seed);
let privkey = ZcashAccountPrivKey.fromExtendedPrivkey(key: accountExtendedPrivKey);
// This "golden" array of bytes was obtained from the original API.
let expected_bytes: [UInt8] = [149, 166, 0, 189, 51, 113, 120, 140, 117, 138, 197, 190, 79, 199, 36, 207, 244, 33, 243, 51, 208, 246, 123, 28, 7, 59, 51, 23, 198, 47, 254, 133, 152, 104, 37, 20, 245, 17, 68, 127, 3, 242, 44, 136, 77, 60, 173, 143, 250, 10, 6, 187, 211, 123, 199, 44, 60, 30, 76, 229, 192, 202, 104, 8]
assert(privkey.toBytes() == expected_bytes);