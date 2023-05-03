import uniffi.zcash.*

val supp = TestSupport.fromCsvFile()


class TransactionBuilderTest(supp: TestSupport) {

    val supp = supp

    fun testTransparentWithNonStandardFees(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_non_standard_fee")
            
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))
        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(200))

        val prover = ZcashLocalTxProver.bundled()

        val feeRule = ZcashFeeRules.FixedNonStandard(0u)

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
    }

    fun testTransparentWithStandardFees(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(1200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(200))

        val prover = ZcashLocalTxProver.bundled()

        val feeRule = ZcashFeeRules.FixedStandard

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testTransparentWithZip317StandardFee(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_zip317_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(19200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(9200))

        val prover = ZcashLocalTxProver.bundled()

        val feeRule = ZcashFeeRules.Zip317Standard

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testTransparentWithZip317NonStandardFee(){
        val expectedTransactionBytes = supp.getAsU8Array(
            "transaction_zip317_non_standard_fee")

        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val address = key.transparent().toAccountPubkey().deriveExternalIvk().deriveAddress(0u)

        val prevCoin = ZcashTxOut(ZcashAmount(19200), address.script())

        val secretKey = key.transparent().deriveExternalSecretKey(0u)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addTransparentInput(secretKey, ZcashOutPoint(List(32) { 0u }, 1u),  prevCoin)
        builder.addTransparentOutput(address, ZcashAmount(9200))

        val prover = ZcashLocalTxProver.bundled()

        val feeRule = ZcashFeeRules.Zip317NonStandard(5000u, 2u, 150u, 34u)

        val result = builder.build(prover, feeRule)

        assert(result.transaction.toBytes() == expectedTransactionBytes)
        
    }

    fun testSaplingWithNonStandardFees(){
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val extsk = key.sapling()
        val paymentAddress = extsk.defaultAddress().address
        val rseed = ZcashRseed.AfterZip212(List(32) { 0u })
        val note = paymentAddress.createNote(200u, rseed)
        val tree = ZcashCommitmentTree.empty()
        tree.append(ZcashSaplingNode.fromCmu(note.cmu()))
        val witness = ZcashIncrementalWitness.fromTree(tree)

        val builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820u))

        builder.addSaplingSpend(extsk, paymentAddress.diversifier(), note, witness.path()!!)

        val ovk = key.sapling().toDiversifiableFullViewingKey().toOvk(ZcashScope.INTERNAL)
        builder.addSaplingOutput(
            ovk, paymentAddress, ZcashAmount(200), ZcashMemoBytes.empty())

        val prover = ZcashLocalTxProver.bundled()
        val feeRule = ZcashFeeRules.FixedNonStandard(0u)

        val result = builder.build(prover, feeRule)
        // The output of each Sapling transaction differs each time.
        // This asserts the size, as its deterministic.
        assert(result.transaction.toBytes().size == 2377)
    }

    fun execute(){
        testTransparentWithNonStandardFees()
        testTransparentWithStandardFees()
        testTransparentWithZip317StandardFee()
        testTransparentWithZip317NonStandardFee()
        testSaplingWithNonStandardFees()
    }
    
}
TransactionBuilderTest(supp).execute()


class OrchardTransactionBuilderTest(supp: TestSupport) {

    val supp = supp

    fun testTransactionGeneration(){
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("unified_spending_key"))

        val ufvk = key.toUnifiedFullViewingKey()
        val fvk = ufvk.orchard()
        val ovk = fvk!!.toOvk(ZcashOrchardScope.EXTERNAL)
        val address = fvk!!.toIvk(ZcashOrchardScope.INTERNAL).address(ZcashOrchardDiversifier.fromBytes(List(11) { 0u }))

        // Note construction
        val noteValue = ZcashOrchardNoteValue.fromRaw(15u)
        val nullifier = ZcashOrchardNullifier.fromBytes(List(32) { 0u })
        val rseed = ZcashOrchardRandomSeed.fromBytes(List(32) { 0u }, nullifier)
        val note = ZcashOrchardNote.fromParts(address, noteValue, nullifier, rseed)

        val merkleHash = ZcashOrchardMerkleHash.fromBytes(List(32) { 0u })
        val authPath = List(32) { merkleHash }
        val merklePath = ZcashOrchardMerklePath.fromParts(0u, authPath)

        val anchor = merklePath.root(note.commitment().toExtractedNoteCommitment())
        val flags = ZcashOrchardFlags.fromParts(true, true)

        val builder = ZcashOrchardTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(
            2030820u), ZcashBlockHeight(2030820u+100u), anchor, flags)
        builder.addSpend(fvk!!, note, merklePath)
        builder.addRecipient(ovk, address, 15u, null)

        val transaction = builder.build(listOf(key.orchard()), List(32) { 0u })

        assert(transaction.toBytes().size == 9165)

    }

    fun execute(){
       testTransactionGeneration()
    }
}
OrchardTransactionBuilderTest(supp).execute()

class TransactionSerializationTest(supp: TestSupport) {

    val supp = supp

    fun testTransactionFromBytes(){       
        val transactionBytes = supp.getAsU8Array("transaction_non_standard_fee")
        ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)
    }

    fun execute(){
        testTransactionFromBytes()
    }
}
TransactionSerializationTest(supp).execute()

class TransactionExplorationTest(supp: TestSupport) {

    val supp = supp

    fun testFirstLevelFields(){       
        val transactionBytes = supp.getAsU8Array("transaction_standard_fee")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        // Id
        val idExpectedBytes = supp.getAsU8Array("transaction_standard_fee_id")
        assert(idExpectedBytes == tx.txid().toBytes())

        // Version
        val versionExpectedBytes = supp.getAsU8Array("transaction_standard_fee_version")
        assert(versionExpectedBytes == tx.version().toBytes())

        // lock time
        assert(0u == tx.lockTime())

        // expiry height
        assert(2030840u == tx.expiryHeight().value())
    }

    fun testTransparentBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_standard_fee")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.transparentBundle()!!

        assert(bundle.isCoinbase() == false)

        // vout
        val vout = bundle.vout()

        assert(1 == vout.size)
        assert(200.toLong() == vout[0].value().value())

        val vout_0_bytes = supp.getAsU8Array("transaction_standard_fee_vout_0")
        assert(vout_0_bytes == vout[0].toBytes())

        val vout_0_address = supp.getAsU8Array("transaction_standard_fee_vout_0_recipient_address")
        assert(vout_0_address == vout[0].recipientAddress()!!.toBytes())

        val script_bytes = supp.getAsU8Array("transaction_standard_fee_vout_0_script")
        assert(script_bytes == vout[0].scriptPubkey().toBytes())

        // vin
        val vin = bundle.vin()

        assert(1 == vin.size)
        val vin0_bytes = supp.getAsU8Array("transaction_standard_fee_vin_0")
        assert(vin0_bytes == vin[0].toBytes())
    }

    fun testSaplingBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_sapling")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.saplingBundle()!!

        // Shielded spends
        val spends = bundle.shieldedSpends()
        assert(1 == spends.size)
        val theSpend = spends[0]
        assert(supp.getAsU8Array("transaction_sapling_spend_0_cv") == theSpend.cv().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_anchor") == theSpend.anchor())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_nullifier") == theSpend.nullifier().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_spend_0_rk") == theSpend.rk().toBytes())

        // Shielded outputs
        val outputs = bundle.shieldedOutputs()
        assert(2 == outputs.size)
        val theOutput = outputs[0]
        assert(supp.getAsU8Array("transaction_sapling_output_0_cv") == theOutput.cv().toBytes())
        assert(supp.getAsU8Array("transaction_sapling_output_0_cmu") == theOutput.cmu().toBytes())

        // Value balance
        assert(0.toLong() == bundle.valueBalance().value())
    }

    fun testOrchardBundle(){       
        val transactionBytes = supp.getAsU8Array("transaction_orchard")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.orchardBundle()!!

        // Actions
        val actions = bundle.actions()
        assert(2 == actions.size)

        val theAction = actions[1]

        assert(supp.getAsU8Array("transaction_orchard_action_1_nullifier") == theAction.nullifier().toBytes())
        assert(supp.getAsU8Array("transaction_orchard_action_1_cmx") == theAction.cmx().toBytes())
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_epk_bytes") == theAction.encryptedNote().epkBytes)
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_enc_ciphertext") == theAction.encryptedNote().encCiphertext)
        assert(supp.getAsU8Array("transaction_orchard_action_1_encrypted_note_out_ciphertext") == theAction.encryptedNote().outCiphertext)
        assert(supp.getAsU8Array("transaction_orchard_action_1_cv_net") == theAction.cvNet().toBytes())

        // Flags
        assert(supp.getAsU8Array("transaction_orchard_flags") == listOf(bundle.flags().toByte()))

        // Value balance
        assert(0.toLong() == bundle.valueBalance().value())

        // Anchor
        assert(supp.getAsU8Array("transaction_orchard_anchor") == bundle.anchor().toBytes())        
    }

    fun testOrchardBundleCrypto(){       
        
        val key = ZcashUnifiedSpendingKey.fromBytes(ZcashKeysEra.ORCHARD, supp.getAsU8Array("testnet_unified_spending_key"))

        val transactionBytes = supp.getAsU8Array("testnet_transaction_orchard")
        val tx = ZcashTransaction.fromBytes(transactionBytes, ZcashBranchId.NU5)

        val bundle = tx.orchardBundle()!!

        // // Verify proof
        // val verifyingKey = ZcashVerifyingKey()
        // bundle.verifyProof(verifyingKey)

        // Decrypt output with IVK
        val ivk = key.toUnifiedFullViewingKey().orchard()!!.toIvk(ZcashOrchardScope.INTERNAL)
        val output_ivk = bundle.decryptOutputWithKey(0u, ivk)
        assert(1999000.toULong() == output_ivk.note.value().value())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == output_ivk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == output_ivk.data)

        // Decrypt output with IVKs
        val outputs_ivk = bundle.decryptOutputWithKeys(listOf(ivk))
        assert(1 == outputs_ivk.size)
        val theOutput_ivk = outputs_ivk[0]
        assert(0.toULong() == theOutput_ivk.idx)
        assert(1999000.toULong() == theOutput_ivk.note.value().value())
        assert(ivk.toBytes() == theOutput_ivk.key.toBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == theOutput_ivk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == theOutput_ivk.data)

        // Decrypt output with OVK
        val ovk = key.toUnifiedFullViewingKey().orchard()!!.toOvk(ZcashOrchardScope.INTERNAL)
        val output_ovk = bundle.recoverOutputWithOvk(0u, ovk)
        assert(1999000.toULong() == output_ovk.note.value().value())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == output_ovk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == output_ovk.data)

        // Decrypt output with OVKs
        val outputs_ovk = bundle.recoverOutputsWithOvks(listOf(ovk))
        assert(1 == outputs_ovk.size)
        val theOutput_ovk = outputs_ovk[0]
        assert(0.toULong() == theOutput_ovk.idx)
        assert(1999000.toULong() == theOutput_ovk.note.value().value())
        assert(ovk.toBytes() == theOutput_ovk.key.toBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_address") == theOutput_ovk.address.toRawAddressBytes())
        assert(supp.getAsU8Array("testnet_transaction_orchard_memo") == theOutput_ovk.data)
    }

    fun execute(){
        testFirstLevelFields()
        testTransparentBundle()
        testSaplingBundle()
        testOrchardBundle()
        testOrchardBundleCrypto()
    }
}
TransactionExplorationTest(supp).execute()