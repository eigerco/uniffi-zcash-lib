require "test/unit"
require "zcash"

class TransactionBuilderTest < Test::Unit::TestCase
    def test_transparent_with_non_standard_fees
        zts = Zcash::TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array(
            "transaction_non_standard_fee")
       
        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD, zts.get_as_u8_array("unified_spending_key"))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = Zcash::ZcashTxOut.new(Zcash::ZcashAmount.new(200), address.script())
        
        secret_key = key.transparent().derive_external_secret_key(0)

        builder = Zcash::ZcashTransactionBuilder.new(
            Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(2030820))

        builder.add_transparent_input(
            secret_key, Zcash::ZcashOutPoint.new([0] * 32, 1),  prev_coin)

        builder.add_transparent_output(address, Zcash::ZcashAmount.new(200))

        prover = Zcash::ZcashLocalTxProver::with_default_location()

        fee_rule = Zcash::ZcashFeeRules::FIXED_NON_STANDARD.new(0)

        result = builder.build(prover, fee_rule)
        
        assert_equal(result.transaction.to_bytes(), expected_transaction_bytes)
    end
    def test_transparent_with_standard_fees
        zts = Zcash::TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array(
            "transaction_standard_fee")
        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD, zts.get_as_u8_array("unified_spending_key"))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = Zcash::ZcashTxOut.new(Zcash::ZcashAmount.new(1200), address.script())
        
        secret_key = key.transparent().derive_external_secret_key(0)

        builder = Zcash::ZcashTransactionBuilder.new(
            Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(2030820))

        builder.add_transparent_input(
            secret_key, Zcash::ZcashOutPoint.new([0] * 32, 1),  prev_coin)

        builder.add_transparent_output(address, Zcash::ZcashAmount.new(200))

        prover = Zcash::ZcashLocalTxProver::with_default_location()

        fee_rule = Zcash::ZcashFeeRules::FIXED_STANDARD.new()

        result = builder.build(prover, fee_rule)

        assert_equal(result.transaction.to_bytes(), expected_transaction_bytes)

    end
    def test_transparent_with_zip317_standard_fee
        zts = Zcash::TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array(
            "transaction_zip317_standard_fee")
        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD, zts.get_as_u8_array("unified_spending_key"))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = Zcash::ZcashTxOut.new(Zcash::ZcashAmount.new(19200), address.script())
        
        secret_key = key.transparent().derive_external_secret_key(0)

        builder = Zcash::ZcashTransactionBuilder.new(
            Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(2030820))

        builder.add_transparent_input(
            secret_key, Zcash::ZcashOutPoint.new([0] * 32, 1),  prev_coin)

        builder.add_transparent_output(address, Zcash::ZcashAmount.new(9200))

        prover = Zcash::ZcashLocalTxProver::with_default_location()

        fee_rule = Zcash::ZcashFeeRules::ZIP317_STANDARD.new()

        result = builder.build(prover, fee_rule)

        assert_equal(result.transaction.to_bytes(), expected_transaction_bytes)

    end
    def test_transparent_with_zip317_non_standard_fee
        zts = Zcash::TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array(
            "transaction_zip317_non_standard_fee")
        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD, zts.get_as_u8_array("unified_spending_key"))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = Zcash::ZcashTxOut.new(Zcash::ZcashAmount.new(19200), address.script())
        
        secret_key = key.transparent().derive_external_secret_key(0)

        builder = Zcash::ZcashTransactionBuilder.new(
            Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(2030820))

        builder.add_transparent_input(
            secret_key, Zcash::ZcashOutPoint.new([0] * 32, 1),  prev_coin)

        builder.add_transparent_output(address, Zcash::ZcashAmount.new(9200))

        prover = Zcash::ZcashLocalTxProver::with_default_location()

        fee_rule = Zcash::ZcashFeeRules::ZIP317_NON_STANDARD.new(5000, 2, 150, 34)

        result = builder.build(prover, fee_rule)

        assert_equal(result.transaction.to_bytes(), expected_transaction_bytes)

    end
    def test_sapling_with_non_standard_fees
        zts = Zcash::TestSupport.from_csv_file()

        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD,
                                                 zts.get_as_u8_array("unified_spending_key"))

        extsk = key.sapling()
        payment_address = extsk.default_address().address
        rseed = Zcash::ZcashRseed::AFTER_ZIP212.new([0] * 32)
        note = payment_address.create_note(200, rseed)
        tree = Zcash::ZcashCommitmentTree::empty()
        tree.append(Zcash::ZcashSaplingNode::from_cmu(note.cmu()))
        witness = Zcash::ZcashIncrementalWitness::from_tree(tree)

        builder = Zcash::ZcashTransactionBuilder.new(
            Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(2030820))

        builder.add_sapling_spend(
            extsk, payment_address.diversifier(), note, witness.path())
        ovk = key.sapling().to_diversifiable_full_viewing_key().to_ovk(Zcash::ZcashScope::INTERNAL)
        builder.add_sapling_output(
            ovk, payment_address, Zcash::ZcashAmount.new(200), Zcash::ZcashMemoBytes.empty())

        prover = Zcash::ZcashLocalTxProver.with_default_location()
        fee_rule = Zcash::ZcashFeeRules::FIXED_NON_STANDARD.new(0)

        result = builder.build(prover, fee_rule)
        # The output of each Sapling transaction differs each time.
        # This asserts the size, as its deterministic.
        assert_equal(result.transaction.to_bytes().length(), 2377)

    end
end

class OrchardTransactionBuilderTest < Test::Unit::TestCase
    def test_transaction_generation
        zts = Zcash::TestSupport.from_csv_file()

        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD,
                                                 zts.get_as_u8_array("unified_spending_key"))

        ufvk = key.to_unified_full_viewing_key()
        fvk = ufvk.orchard()
        ovk = fvk.to_ovk(Zcash::ZcashOrchardScope::EXTERNAL)
        address = fvk.to_ivk(Zcash::ZcashOrchardScope::INTERNAL).address(
            Zcash::ZcashOrchardDiversifier::from_bytes([0] * 11))

        # Note construction
        note_value = Zcash::ZcashOrchardNoteValue::from_raw(15)
        nullifier = Zcash::ZcashOrchardNullifier::from_bytes([0] * 32)
        rseed = Zcash::ZcashOrchardRandomSeed::from_bytes([0] * 32, nullifier)
        note = Zcash::ZcashOrchardNote::from_parts(
            address, note_value, nullifier, rseed)

        auth_path = [Zcash::ZcashOrchardMerkleHash::from_bytes([0] * 32)] * 32
        merkle_path = Zcash::ZcashOrchardMerklePath::from_parts(0, auth_path)

        anchor = merkle_path.root(
            note.commitment().to_extracted_note_commitment())
        flags = Zcash::ZcashOrchardFlags::from_parts(true, true)

        builder = Zcash::ZcashOrchardTransactionBuilder.new(Zcash::ZcashConsensusParameters::MAIN_NETWORK, Zcash::ZcashBlockHeight.new(
            2030820), Zcash::ZcashBlockHeight.new(2030820+100), anchor, flags)
        builder.add_spend(fvk, note, merkle_path)
        builder.add_recipient(ovk, address, 15, nil)

        transaction = builder.build([key.orchard()], [0]*32)

        assert_equal(transaction.to_bytes().length(), 9165)
    end
end

class TransactionSerializationTest < Test::Unit::TestCase
    def test_transaction_from_bytes
        zts = Zcash::TestSupport.from_csv_file()

        transaction_bytes = zts.get_as_u8_array("transaction_non_standard_fee")

        Zcash::ZcashTransaction.from_bytes(transaction_bytes, Zcash::ZcashBranchId::NU5)
    end
end

class TransactionExplorationTest < Test::Unit::TestCase
    def test_first_level_fields
        zts = Zcash::TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_standard_fee")
        tx = Zcash::ZcashTransaction.from_bytes(tx_bytes, Zcash::ZcashBranchId::NU5)
        
        # Id
        id_expected_bytes = zts.get_as_u8_array("transaction_standard_fee_id")
        assert_equal(id_expected_bytes, tx.txid().to_bytes())

        # Version
        version_expected_bytes = zts.get_as_u8_array(
            "transaction_standard_fee_version")
        assert_equal(version_expected_bytes, tx.version().to_bytes())

        # lock time
        assert_equal(0, tx.lock_time())

        # expiry height
        assert_equal(2030840, tx.expiry_height().value())
    end
    def test_transparent_bundle
        zts = Zcash::TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_standard_fee")
        tx = Zcash::ZcashTransaction.from_bytes(tx_bytes, Zcash::ZcashBranchId::NU5)

        bundle = tx.transparent_bundle()

        assert_false(bundle.is_coinbase())

        # vout
        vout = bundle.vout()

        assert_equal(1, vout.length())
        assert_equal(200, vout[0].value().value())

        vout_0_bytes = zts.get_as_u8_array(
            "transaction_standard_fee_vout_0")
        assert_equal(vout_0_bytes, vout[0].to_bytes())
        

        vout_0_address = zts.get_as_u8_array(
            "transaction_standard_fee_vout_0_recipient_address")
        assert_equal(
            vout_0_address, vout[0].recipient_address().to_bytes())

        script_bytes = zts.get_as_u8_array(
            "transaction_standard_fee_vout_0_script")
        assert_equal(script_bytes, vout[0].script_pubkey().to_bytes())

        # vin
        vin = bundle.vin()

        assert_equal(1, vin.length())
        vin0_bytes = zts.get_as_u8_array("transaction_standard_fee_vin_0")
        assert_equal(vin0_bytes, vin[0].to_bytes())
    end
    def test_sapling_bundle
        zts = Zcash::TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_sapling")
        tx = Zcash::ZcashTransaction.from_bytes(tx_bytes, Zcash::ZcashBranchId::NU5)

        bundle = tx.sapling_bundle()

        # Shielded spends
        spends = bundle.shielded_spends()
        assert_equal(1, spends.length())
        the_spend = spends[0]
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_spend_0_cv"), the_spend.cv().to_bytes())
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_spend_0_anchor"), the_spend.anchor())
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_spend_0_nullifier"), the_spend.nullifier().to_bytes())
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_spend_0_rk"), the_spend.rk().to_bytes())

        # Shielded outputs
        outputs = bundle.shielded_outputs()
        assert_equal(1, spends.length())
        the_output = outputs[0]
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_output_0_cv"), the_output.cv().to_bytes())
        assert_equal(zts.get_as_u8_array(
            "transaction_sapling_output_0_cmu"), the_output.cmu().to_bytes())

        # Value balance
        assert_equal(0, bundle.value_balance().value())
    end
    def test_orchard_bundle
        zts = Zcash::TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_orchard")
        tx = Zcash::ZcashTransaction::from_bytes(tx_bytes, Zcash::ZcashBranchId::NU5)

        bundle = tx.orchard_bundle()

        # Actions
        actions = bundle.actions()
        assert_equal(2, actions.length())

        the_action = actions[1]

        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_nullifier"), the_action.nullifier().to_bytes())
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_cmx"), the_action.cmx().to_bytes())
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_encrypted_note_epk_bytes"), the_action.encrypted_note().epk_bytes)
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_encrypted_note_enc_ciphertext"), the_action.encrypted_note().enc_ciphertext)
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_encrypted_note_out_ciphertext"), the_action.encrypted_note().out_ciphertext)
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_action_1_cv_net"), the_action.cv_net().to_bytes())

        # Flags
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_flags"), [bundle.flags().to_byte()])

        # Value balance
        assert_equal(0, bundle.value_balance().value())

        # Anchor
        assert_equal(zts.get_as_u8_array(
            "transaction_orchard_anchor"), bundle.anchor().to_bytes())
    end
    def test_orchard_bundle_crypto
        zts = Zcash::TestSupport.from_csv_file()
        key = Zcash::ZcashUnifiedSpendingKey.from_bytes(Zcash::ZcashKeysEra::ORCHARD,
                                                 zts.get_as_u8_array("testnet_unified_spending_key"))

        tx_bytes = zts.get_as_u8_array("testnet_transaction_orchard")
        tx = Zcash::ZcashTransaction.from_bytes(tx_bytes, Zcash::ZcashBranchId::NU5)

        bundle = tx.orchard_bundle()

        # # Verify proof
        # verifying_key = Zcash::ZcashVerifyingKey.new()
        # bundle.verify_proof(verifying_key)

        # Decrypt output with IVK
        ivk = key.to_unified_full_viewing_key().orchard().to_ivk(Zcash::ZcashOrchardScope::INTERNAL)
        output = bundle.decrypt_output_with_key(0, ivk)
        assert_equal(1999000, output.note.value().value())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_address"), output.address.to_raw_address_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_memo"), output.data)

        # Decrypt output with IVKs
        outputs = bundle.decrypt_output_with_keys([ivk])
        assert_equal(1, outputs.length())
        the_output = outputs[0]
        assert_equal(0, the_output.idx)
        assert_equal(1999000, the_output.note.value().value())
        assert_equal(ivk.to_bytes(), the_output.key.to_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_address"), the_output.address.to_raw_address_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_memo"), the_output.data)

        # Decrypt output with OVK
        ovk = key.to_unified_full_viewing_key().orchard().to_ovk(Zcash::ZcashOrchardScope::INTERNAL)
        output = bundle.recover_output_with_ovk(0, ovk)
        assert_equal(1999000, output.note.value().value())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_address"), output.address.to_raw_address_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_memo"), output.data)

        # Decrypt output with OVKs
        outputs = bundle.recover_outputs_with_ovks([ovk])
        assert_equal(1, outputs.length())
        the_output = outputs[0]
        assert_equal(0, the_output.idx)
        assert_equal(1999000, the_output.note.value().value())
        assert_equal(ovk.to_bytes(), the_output.key.to_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_address"), the_output.address.to_raw_address_bytes())
        assert_equal(zts.get_as_u8_array(
            "testnet_transaction_orchard_memo"), the_output.data)
    end
end
