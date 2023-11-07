import zcash

let zts = TestSupport.fromCsvFile()

class TransactionBuilderTest {
    let zts: TestSupport
    init(zts: TestSupport) {
        self.zts = zts
    }

    func testTransparentWithNonStandardFees() {
        let expectedTransactionBytes = zts.getAsU8Array(key: "transaction_non_standard_fee")

        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))
        let address = try! key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(childIndex: 0)

        let prevCoin = ZcashTxOut(value: try! ZcashAmount(amount: 200), scriptPubkey: address.script())

        let secretKey = try! key.transparent().deriveExternalSecretKey(childIndex: 0)

        let builder = ZcashTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, blockHeight: ZcashBlockHeight(v: 2_030_820))

        let outPoint = try! ZcashOutPoint(hash: Array(repeating: 0, count: 32), n: 1)

        builder.addTransparentInput(sk: secretKey, utxo: outPoint, coin: prevCoin)

        try! builder.addTransparentOutput(to: address, value: ZcashAmount(amount: 200))

        let prover = try! ZcashLocalTxProver.withDefaultLocation()

        let feeRule = ZcashFeeRules.fixedNonStandard(amount: 0)

        let result = try! builder.build(prover: prover, feeRule: feeRule)

        assert(try! result.transaction.toBytes() == expectedTransactionBytes)
    }

    func testTransparentWithStandardFees() {
        let expectedTransactionBytes = zts.getAsU8Array(key: "transaction_standard_fee")

        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))
        let address = try! key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(childIndex: 0)

        let prevCoin = ZcashTxOut(value: try! ZcashAmount(amount: 10200), scriptPubkey: address.script())

        let secretKey = try! key.transparent().deriveExternalSecretKey(childIndex: 0)

        let builder = ZcashTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, blockHeight: ZcashBlockHeight(v: 2_030_820))

        let outPoint = try! ZcashOutPoint(hash: Array(repeating: 0, count: 32), n: 1)

        builder.addTransparentInput(sk: secretKey, utxo: outPoint, coin: prevCoin)

        try! builder.addTransparentOutput(to: address, value: ZcashAmount(amount: 200))

        let prover = try! ZcashLocalTxProver.withDefaultLocation()

        let feeRule = ZcashFeeRules.fixedStandard

        let result = try! builder.build(prover: prover, feeRule: feeRule)

        assert(try! result.transaction.toBytes() == expectedTransactionBytes)
    }

    func testTransparentWithZip317StandardFee() {
        let expectedTransactionBytes = zts.getAsU8Array(key: "transaction_zip317_standard_fee")

        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))
        let address = try! key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(childIndex: 0)

        let prevCoin = ZcashTxOut(value: try! ZcashAmount(amount: 19200), scriptPubkey: address.script())

        let secretKey = try! key.transparent().deriveExternalSecretKey(childIndex: 0)

        let builder = ZcashTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, blockHeight: ZcashBlockHeight(v: 2_030_820))

        let outPoint = try! ZcashOutPoint(hash: Array(repeating: 0, count: 32), n: 1)

        builder.addTransparentInput(sk: secretKey, utxo: outPoint, coin: prevCoin)

        try! builder.addTransparentOutput(to: address, value: ZcashAmount(amount: 9200))

        let prover = try! ZcashLocalTxProver.withDefaultLocation()

        let feeRule = ZcashFeeRules.zip317Standard

        let result = try! builder.build(prover: prover, feeRule: feeRule)

        assert(try! result.transaction.toBytes() == expectedTransactionBytes)
    }

    func testTransparentWithZip317NonStandardFee() {
        let expectedTransactionBytes = zts.getAsU8Array(key: "transaction_zip317_non_standard_fee")

        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))
        let address = try! key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(childIndex: 0)

        let prevCoin = ZcashTxOut(value: try! ZcashAmount(amount: 19200), scriptPubkey: address.script())

        let secretKey = try! key.transparent().deriveExternalSecretKey(childIndex: 0)

        let builder = ZcashTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, blockHeight: ZcashBlockHeight(v: 2_030_820))

        let outPoint = try! ZcashOutPoint(hash: Array(repeating: 0, count: 32), n: 1)

        builder.addTransparentInput(sk: secretKey, utxo: outPoint, coin: prevCoin)

        try! builder.addTransparentOutput(to: address, value: ZcashAmount(amount: 9200))

        let prover = try! ZcashLocalTxProver.withDefaultLocation()

        let feeRule = ZcashFeeRules.zip317NonStandard(marginalFee: 5000, graceActions: 2, p2pkhStandardInputSize: 150, p2pkhStandardOutputSize: 34)

        let result = try! builder.build(prover: prover, feeRule: feeRule)

        assert(try! result.transaction.toBytes() == expectedTransactionBytes)
    }

    func testSaplingWithNonStandardFees() {
        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))

        let extsk = key.sapling()
        let paymentAddress = extsk.defaultAddress().address
        let rseed = ZcashRseed.afterZip212(data: Array(repeating: 0, count: 32))
        let note = try! paymentAddress.createNote(value: 200, rseed: rseed)
        let tree = ZcashCommitmentTree.empty()
        try! tree.append(node: ZcashSaplingNode.fromCmu(cmu: note.cmu()))
        let witness = ZcashIncrementalWitness.fromTree(tree: tree)

        let builder = ZcashTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, blockHeight: ZcashBlockHeight(v: 2_030_820))

        builder.addSaplingSpend(extsk: extsk, diversifier: paymentAddress.diversifier(), note: note, merklePath: witness.path()!)

        let ovk = key.sapling().toDiversifiableFullViewingKey().toOvk(scope: ZcashScope.internal)
        builder.addSaplingOutput(ovk: ovk, to: paymentAddress, value: try! ZcashAmount(amount: 200), memo: ZcashMemoBytes.empty())

        let prover = try! ZcashLocalTxProver.withDefaultLocation()
        let feeRule = ZcashFeeRules.fixedNonStandard(amount: 0)

        let result = try! builder.build(prover: prover, feeRule: feeRule)
        // The output of each Sapling transaction differs each time.
        // This asserts the size, as its deterministic.
        assert(try! result.transaction.toBytes().count == 2377)
    }

    func execute() {
        testTransparentWithNonStandardFees()
        testTransparentWithStandardFees()
        testTransparentWithZip317StandardFee()
        testTransparentWithZip317NonStandardFee()
        testSaplingWithNonStandardFees()
    }
}

TransactionBuilderTest(zts: zts).execute()

class OrchardTransactionBuilderTest {
    let zts: TestSupport
    init(zts: TestSupport) {
        self.zts = zts
    }

    func testTransactionGeneration() {
        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "unified_spending_key"))
        let ufvk = key.toUnifiedFullViewingKey()
        let fvk = ufvk.orchard()!
        let ovk = fvk.toOvk(scope: ZcashOrchardScope.external)
        let address = fvk.toIvk(scope: ZcashOrchardScope.internal).address(diversifier: try! ZcashOrchardDiversifier.fromBytes(bytes: Array(repeating: 0, count: 11)))

        // Note construction
        let noteValue = ZcashOrchardNoteValue.fromRaw(value: 15)
        let nullifier = try! ZcashOrchardNullifier.fromBytes(data: Array(repeating: 0, count: 32))
        let rseed = try! ZcashOrchardRandomSeed.fromBytes(data: Array(repeating: 0, count: 32), rho: nullifier)
        let note = try! ZcashOrchardNote.fromParts(recipient: address, value: noteValue, rho: nullifier, rseed: rseed)

        let merkleHash = try! ZcashOrchardMerkleHash.fromBytes(data: Array(repeating: 0, count: 32))
        let authPath = Array(repeating: merkleHash, count: 32)
        let merklePath = try! ZcashOrchardMerklePath.fromParts(position: 0, authPath: authPath)

        let anchor = merklePath.root(cmx: note.commitment().toExtractedNoteCommitment())
        let flags = ZcashOrchardFlags.fromParts(spendsEnabled: true, outputsEnabled: true)

        let builder = ZcashOrchardTransactionBuilder(parameters: ZcashConsensusParameters.mainNetwork, targetHeight: ZcashBlockHeight(v: 2_030_820), expiryHeight: ZcashBlockHeight(v: 2_030_820 + 100), anchor: anchor, flags: flags)
        builder.addSpend(fvk: fvk, note: note, merklePath: merklePath)
        try! builder.addRecipient(ovk: ovk, recipient: address, value: noteValue, memo: nil)

        let transaction = try! builder.build(keys: [key.orchard()], sighash: Array(repeating: 0, count: 32))

        assert(try! transaction.toBytes().count == 9165)
    }

    func execute() {
        testTransactionGeneration()
    }
}

OrchardTransactionBuilderTest(zts: zts).execute()

class TransactionSerializationTest {
    let zts: TestSupport
    init(zts: TestSupport) {
        self.zts = zts
    }

    func testTransactionFromBytes() {
        let transactionBytes = zts.getAsU8Array(key: "transaction_non_standard_fee")
        _ = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)
    }

    func execute() {
        testTransactionFromBytes()
    }
}

TransactionSerializationTest(zts: zts).execute()

class TransactionExplorationTest {
    let zts: TestSupport
    init(zts: TestSupport) {
        self.zts = zts
    }

    func testFirstLevelFields() {
        let transactionBytes = zts.getAsU8Array(key: "transaction_standard_fee")
        let tx = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)

        // Id
        let idExpectedBytes = zts.getAsU8Array(key: "transaction_standard_fee_id")
        let idBytes = try! tx.txid().toBytes()
        assert(idExpectedBytes == idBytes)

        // Version
        let versionExpectedBytes = zts.getAsU8Array(key: "transaction_standard_fee_version")
        let versionBytes = try! tx.version().toBytes()
        assert(versionExpectedBytes == versionBytes)

        // lock time
        assert(tx.lockTime() == 0)

        // expiry height
        assert(tx.expiryHeight().value() == 2_030_860)
    }

    func testTransparentBundle() {
        let transactionBytes = zts.getAsU8Array(key: "transaction_standard_fee")
        let tx = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)

        let bundle = tx.transparentBundle()!

        assert(bundle.isCoinbase() == false)

        // vout
        let vout = bundle.vout()

        assert(vout.count == 1)
        assert(vout[0].value().value() == 200)

        let expected_vout_0_bytes = zts.getAsU8Array(key: "transaction_standard_fee_vout_0")
        let vout_0_bytes = try! vout[0].toBytes()
        assert(expected_vout_0_bytes == vout_0_bytes)

        let expected_vout_0_address = zts.getAsU8Array(key: "transaction_standard_fee_vout_0_recipient_address")
        let vout_0_address = vout[0].recipientAddress()!.toBytes()
        assert(expected_vout_0_address == vout_0_address)

        let expected_script_bytes = zts.getAsU8Array(key: "transaction_standard_fee_vout_0_script")
        let script_bytes = try! vout[0].scriptPubkey().toBytes()
        assert(expected_script_bytes == script_bytes)

        // vin
        let vin = bundle.vin()

        assert(vin.count == 1)
        let expected_vin0_bytes = zts.getAsU8Array(key: "transaction_standard_fee_vin_0")
        let vin0_bytes = try! vin[0].toBytes()
        assert(expected_vin0_bytes == vin0_bytes)
    }

    func testSaplingBundle() {
        let transactionBytes = zts.getAsU8Array(key: "transaction_sapling")
        let tx = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)

        let bundle = tx.saplingBundle()!

        // Shielded spends
        let spends = bundle.shieldedSpends()
        assert(spends.count == 1)
        let theSpend = spends[0]
        assert(zts.getAsU8Array(key: "transaction_sapling_spend_0_cv") == theSpend.cv().toBytes())
        assert(zts.getAsU8Array(key: "transaction_sapling_spend_0_anchor") == theSpend.anchor())
        assert(zts.getAsU8Array(key: "transaction_sapling_spend_0_nullifier") == theSpend.nullifier().toBytes())
        assert(zts.getAsU8Array(key: "transaction_sapling_spend_0_rk") == (try! theSpend.rk().toBytes()))

        // Shielded outputs
        let outputs = bundle.shieldedOutputs()
        assert(outputs.count == 2)
        let theOutput = outputs[0]
        assert(zts.getAsU8Array(key: "transaction_sapling_output_0_cv") == theOutput.cv().toBytes())
        assert(zts.getAsU8Array(key: "transaction_sapling_output_0_cmu") == theOutput.cmu().toBytes())

        // Value balance
        assert(bundle.valueBalance().value() == 0)
    }

    func testOrchardBundle() {
        let transactionBytes = zts.getAsU8Array(key: "transaction_orchard")
        let tx = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)

        let bundle = tx.orchardBundle()!

        // Actions
        let actions = bundle.actions()
        assert(actions.count == 2)

        let theAction = actions[1]

        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_nullifier") == theAction.nullifier().toBytes())
        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_cmx") == theAction.cmx().toBytes())
        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_encrypted_note_epk_bytes") == theAction.encryptedNote().epkBytes)
        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_encrypted_note_enc_ciphertext") == theAction.encryptedNote().encCiphertext)
        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_encrypted_note_out_ciphertext") == theAction.encryptedNote().outCiphertext)
        assert(zts.getAsU8Array(key: "transaction_orchard_action_1_cv_net") == theAction.cvNet().toBytes())

        // Flags
        assert(zts.getAsU8Array(key: "transaction_orchard_flags") == [bundle.flags().toByte()])

        // Value balance
        assert(bundle.valueBalance().value() == 0)

        // Anchor
        assert(zts.getAsU8Array(key: "transaction_orchard_anchor") == bundle.anchor().toBytes())
    }

    func testOrchardBundleCrypto() {
        let key = try! ZcashUnifiedSpendingKey.fromBytes(era: ZcashKeysEra.orchard, encoded: zts.getAsU8Array(key: "testnet_unified_spending_key"))
        let transactionBytes = zts.getAsU8Array(key: "testnet_transaction_orchard")
        let tx = try! ZcashTransaction.fromBytes(data: transactionBytes, consensusBranchId: ZcashBranchId.nu5)

        let bundle = tx.orchardBundle()!

        // Verify proof
        // let verifyingKey = ZcashVerifyingKey()
        // _ = try! bundle.verifyProof(key: verifyingKey)

        // Decrypt output with IVK
        let ivk = key.toUnifiedFullViewingKey().orchard()!.toIvk(scope: ZcashOrchardScope.internal)
        let output_ivk = try! bundle.decryptOutputWithKey(actionIdx: 0, ivk: ivk)
        assert(output_ivk.note.value().value() == 1_999_000)
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_address") == output_ivk.address.toRawAddressBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_memo") == output_ivk.data)

        // Decrypt output with IVKs
        let outputs_ivk = bundle.decryptOutputWithKeys(ivks: [ivk])
        assert(outputs_ivk.count == 1)
        let theOutput_ivk = outputs_ivk[0]
        assert(theOutput_ivk.idx == 0)
        assert(theOutput_ivk.note.value().value() == 1_999_000)
        assert(ivk.toBytes() == theOutput_ivk.key.toBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_address") == theOutput_ivk.address.toRawAddressBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_memo") == theOutput_ivk.data)

        // Decrypt output with OVK
        let ovk = key.toUnifiedFullViewingKey().orchard()!.toOvk(scope: ZcashOrchardScope.internal)
        let output_ovk = try! bundle.recoverOutputWithOvk(actionIdx: 0, ovk: ovk)
        assert(output_ovk.note.value().value() == 1_999_000)
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_address") == output_ovk.address.toRawAddressBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_memo") == output_ovk.data)

        // Decrypt output with OVKs
        let outputs_ovk = bundle.recoverOutputsWithOvks(ovks: [ovk])
        assert(outputs_ovk.count == 1)
        let theOutput_ovk = outputs_ovk[0]
        assert(theOutput_ovk.idx == 0)
        assert(theOutput_ovk.note.value().value() == 1_999_000)
        assert(ovk.toBytes() == theOutput_ovk.key.toBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_address") == theOutput_ovk.address.toRawAddressBytes())
        assert(zts.getAsU8Array(key: "testnet_transaction_orchard_memo") == theOutput_ovk.data)
    }

    func execute() {
        testFirstLevelFields()
        testTransparentBundle()
        testSaplingBundle()
        testOrchardBundle()
        testOrchardBundleCrypto()
    }
}

TransactionExplorationTest(zts: zts).execute()
