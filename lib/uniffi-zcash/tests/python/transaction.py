import unittest
from zcash import *

class TransactionBuilderTest(unittest.TestCase):
    def test_transparent_with_non_standard_fees(self):
        zts = TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array("transaction_non_standard_fee")
        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = ZcashTxOut(ZcashAmount(200), address.script())

        secret_key = key.transparent().derive_external_secret_key(0)

        builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820))

        builder.add_transparent_input(secret_key, ZcashOutPoint([0] * 32, 1),  prev_coin)
        builder.add_transparent_output(address, ZcashAmount(200))

        prover = ZcashLocalTxProver.bundled()

        fee_rule = ZcashFeeRules.FIXED_NON_STANDARD(0)

        result = builder.build(prover, fee_rule)

        self.assertEqual(result.transaction.to_bytes(), expected_transaction_bytes)

    def test_transparent_with_standard_fees(self):
        zts = TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array("transaction_standard_fee")
        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = ZcashTxOut(ZcashAmount(1200), address.script())

        secret_key = key.transparent().derive_external_secret_key(0)

        builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820))

        builder.add_transparent_input(secret_key, ZcashOutPoint([0] * 32, 1),  prev_coin)
        builder.add_transparent_output(address, ZcashAmount(200))

        prover = ZcashLocalTxProver.bundled()

        fee_rule = ZcashFeeRules.FIXED_STANDARD()

        result = builder.build(prover, fee_rule)

        self.assertEqual(result.transaction.to_bytes(), expected_transaction_bytes)

    def test_transparent_with_zip317_standard_fee(self):
        zts = TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array("transaction_zip317_standard_fee")
        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = ZcashTxOut(ZcashAmount(19200), address.script())

        secret_key = key.transparent().derive_external_secret_key(0)

        builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820))

        builder.add_transparent_input(secret_key, ZcashOutPoint([0] * 32, 1),  prev_coin)
        builder.add_transparent_output(address, ZcashAmount(9200))

        prover = ZcashLocalTxProver.bundled()

        fee_rule = ZcashFeeRules.ZIP317_STANDARD()

        result = builder.build(prover, fee_rule)

        self.assertEqual(result.transaction.to_bytes(), expected_transaction_bytes)

    def test_transparent_with_zip317_non_standard_fee(self):
        zts = TestSupport.from_csv_file()

        expected_transaction_bytes = zts.get_as_u8_array("transaction_zip317_non_standard_fee")
        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        address = key.transparent().to_account_pubkey().derive_external_ivk().derive_address(0)

        prev_coin = ZcashTxOut(ZcashAmount(19200), address.script())

        secret_key = key.transparent().derive_external_secret_key(0)

        builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820))

        builder.add_transparent_input(secret_key, ZcashOutPoint([0] * 32, 1),  prev_coin)
        builder.add_transparent_output(address, ZcashAmount(9200))

        prover = ZcashLocalTxProver.bundled()

        fee_rule = ZcashFeeRules.ZIP317_NON_STANDARD(5000, 2, 150, 34)

        result = builder.build(prover, fee_rule)

        self.assertEqual(result.transaction.to_bytes(), expected_transaction_bytes)

    def test_sapling_with_non_standard_fees(self):
        zts = TestSupport.from_csv_file()

        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        extsk = key.sapling()
        payment_address = extsk.default_address().address
        rseed = ZcashRseed.AFTER_ZIP212([0] * 32)
        note = payment_address.create_note(200, rseed)
        tree = ZcashCommitmentTree.empty()
        tree.append(ZcashSaplingNode.from_cmu(note.cmu()))
        witness = ZcashIncrementalWitness.from_tree(tree)

        builder = ZcashTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820))

        builder.add_sapling_spend(extsk, payment_address.diversifier(), note, witness.path())
        ovk = key.sapling().to_diversifiable_full_viewing_key().to_ovk(ZcashScope.INTERNAL)
        builder.add_sapling_output(ovk, payment_address, ZcashAmount(200), ZcashMemoBytes.empty())


        prover = ZcashLocalTxProver.bundled()
        fee_rule = ZcashFeeRules.FIXED_NON_STANDARD(0)

        result = builder.build(prover, fee_rule)
        # The output of each Sapling transaction differs each time.
        # This asserts the size, as its deterministic.
        self.assertEqual(len(result.transaction.to_bytes()), 2377)

class OrchardTransactionBuilderTest(unittest.TestCase):
     def test_transaction_generation(self):
        zts = TestSupport.from_csv_file()

        key_seed = zts.get_as_u8_array("seed")

        key = ZcashUnifiedSpendingKey.from_seed(
            ZcashConsensusParameters.MAIN_NETWORK,
            key_seed,
            ZcashAccountId(0))

        ufvk = key.to_unified_full_viewing_key()
        fvk = ufvk.orchard()
        ovk = fvk.to_ovk(ZcashOrchardScope.EXTERNAL)
        address = fvk.to_ivk(ZcashOrchardScope.INTERNAL).address(ZcashOrchardDiversifier.from_bytes([0] * 11))

        ## Note construction
        note_value = ZcashOrchardNoteValue.from_raw(15)
        nullifier = ZcashOrchardNullifier.from_bytes([0] * 32)
        rseed = ZcashOrchardRandomSeed.from_bytes([0] * 32, nullifier)
        note = ZcashOrchardNote.from_parts(address, note_value, nullifier, rseed)

        auth_path = [ZcashOrchardMerkleHash.from_bytes([0] * 32)] * 32
        merkle_path = ZcashOrchardMerklePath.from_parts(0, auth_path)


        anchor = merkle_path.root(note.commitment().to_extracted_note_commitment())

        builder = ZcashOrchardTransactionBuilder(ZcashConsensusParameters.MAIN_NETWORK, ZcashBlockHeight(2030820), ZcashBlockHeight(2030820+100), anchor)
        builder.add_spend(fvk, note, merkle_path)
        builder.add_output(ovk, address, 15, None)

        transaction = builder.build([key.orchard()], [0]*32)

        self.assertEqual(len(transaction.to_bytes()), 9165)

class TransactionExplorationTest(unittest.TestCase):
    def test_first_level_fields(self):
        zts = TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_standard_fee")
        tx = ZcashTransaction.from_bytes(tx_bytes, ZcashBranchId.NU5)

        # Id
        id_expected_bytes = zts.get_as_u8_array("transaction_standard_fee_id")
        self.assertEqual(id_expected_bytes, tx.txid().to_bytes())

        # Version
        version_expected_bytes = zts.get_as_u8_array(
            "transaction_standard_fee_version")
        self.assertEqual(version_expected_bytes, tx.version().to_bytes())

        # lock time
        self.assertEqual(0, tx.lock_time())

        # expiry height
        self.assertEqual(2030840, tx.expiry_height().value())

    def test_transparent_bundle(self):

        zts = TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_standard_fee")
        tx = ZcashTransaction.from_bytes(tx_bytes, ZcashBranchId.NU5)

        bundle = tx.transparent_bundle()

        self.assertFalse(bundle.is_coinbase())

    def test_sapling_bundle(self):

        zts = TestSupport.from_csv_file()

        tx_bytes = zts.get_as_u8_array("transaction_sapling")
        tx = ZcashTransaction.from_bytes(tx_bytes, ZcashBranchId.NU5)

        bundle = tx.sapling_bundle()

        # Shielded spends
        spends = bundle.shielded_spends()
        self.assertEqual(1, len(spends))
        the_spend = spends[0]
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_spend_0_cv"), the_spend.cv().to_bytes())
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_spend_0_anchor"), the_spend.anchor())
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_spend_0_nullifier"), the_spend.nullifier().to_bytes())
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_spend_0_rk"), the_spend.rk().to_bytes())

        # Shielded outputs
        outputs = bundle.shielded_outputs()
        self.assertEqual(1, len(spends))
        the_output = outputs[0]
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_output_0_cv"), the_output.cv().to_bytes())
        self.assertEqual(zts.get_as_u8_array(
            "transaction_sapling_output_0_cmu"), the_output.cmu().to_bytes())

        # Value balance
        self.assertEqual(0, bundle.value_balance().value())


if __name__ == '__main__':
    unittest.main()
