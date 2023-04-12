import unittest
from zcash import *

class TransactionBuilderTest(unittest.TestCase):
    def test_transparent_with_non_standard_fees(self):
        expected_transaction_bytes = [5, 0, 0, 128, 10, 39, 167, 38, 180, 208, 214, 194, 0, 0, 0, 0, 248, 252, 30, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 107, 72, 48, 69, 2, 33, 0, 179, 128, 24, 175, 169, 151, 111, 47, 173, 216, 16, 109, 90, 125, 138, 75, 174, 64, 173, 135, 140, 34, 163, 125, 190, 152, 223, 242, 237, 47, 199, 96, 2, 32, 59, 52, 183, 101, 133, 44, 107, 208, 166, 195, 86, 46, 165, 173, 19, 81, 41, 16, 105, 17, 50, 169, 1, 0, 144, 113, 207, 20, 235, 214, 234, 237, 1, 33, 2, 113, 189, 124, 164, 245, 12, 96, 195, 100, 124, 115, 53, 29, 197, 235, 95, 104, 106, 186, 198, 59, 15, 56, 77, 132, 58, 107, 180, 80, 227, 138, 106, 255, 255, 255, 255, 1, 200, 0, 0, 0, 0, 0, 0, 0, 25, 118, 169, 20, 58, 175, 143, 10, 98, 89, 200, 2, 255, 208, 146, 136, 250, 224, 91, 40, 106, 79, 229, 68, 136, 172, 0, 0, 0]
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

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
        expected_transaction_bytes = [5, 0, 0, 128, 10, 39, 167, 38, 180, 208, 214, 194, 0, 0, 0, 0, 248, 252, 30, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 106, 71, 48, 68, 2, 32, 126, 123, 35, 115, 60, 147, 215, 234, 193, 57, 232, 73, 238, 51, 241, 194, 12, 12, 191, 40, 65, 74, 135, 206, 79, 10, 7, 59, 42, 70, 22, 159, 2, 32, 29, 215, 79, 106, 176, 147, 47, 145, 38, 84, 21, 136, 70, 172, 38, 146, 92, 34, 83, 101, 156, 109, 191, 44, 39, 155, 162, 77, 171, 127, 53, 118, 1, 33, 2, 113, 189, 124, 164, 245, 12, 96, 195, 100, 124, 115, 53, 29, 197, 235, 95, 104, 106, 186, 198, 59, 15, 56, 77, 132, 58, 107, 180, 80, 227, 138, 106, 255, 255, 255, 255, 1, 200, 0, 0, 0, 0, 0, 0, 0, 25, 118, 169, 20, 58, 175, 143, 10, 98, 89, 200, 2, 255, 208, 146, 136, 250, 224, 91, 40, 106, 79, 229, 68, 136, 172, 0, 0, 0]
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

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
        expected_transaction_bytes = [5, 0, 0, 128, 10, 39, 167, 38, 180, 208, 214, 194, 0, 0, 0, 0, 248, 252, 30, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 107, 72, 48, 69, 2, 33, 0, 146, 78, 196, 184, 67, 144, 153, 168, 207, 137, 226, 180, 98, 162, 53, 126, 167, 82, 177, 45, 107, 26, 245, 39, 142, 171, 224, 148, 244, 160, 149, 252, 2, 32, 34, 126, 17, 72, 102, 103, 169, 13, 80, 99, 8, 27, 185, 109, 184, 83, 186, 106, 139, 70, 205, 204, 22, 87, 94, 140, 189, 79, 246, 97, 152, 72, 1, 33, 2, 113, 189, 124, 164, 245, 12, 96, 195, 100, 124, 115, 53, 29, 197, 235, 95, 104, 106, 186, 198, 59, 15, 56, 77, 132, 58, 107, 180, 80, 227, 138, 106, 255, 255, 255, 255, 1, 240, 35, 0, 0, 0, 0, 0, 0, 25, 118, 169, 20, 58, 175, 143, 10, 98, 89, 200, 2, 255, 208, 146, 136, 250, 224, 91, 40, 106, 79, 229, 68, 136, 172, 0, 0, 0]
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

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
        expected_transaction_bytes = [5, 0, 0, 128, 10, 39, 167, 38, 180, 208, 214, 194, 0, 0, 0, 0, 248, 252, 30, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 107, 72, 48, 69, 2, 33, 0, 146, 78, 196, 184, 67, 144, 153, 168, 207, 137, 226, 180, 98, 162, 53, 126, 167, 82, 177, 45, 107, 26, 245, 39, 142, 171, 224, 148, 244, 160, 149, 252, 2, 32, 34, 126, 17, 72, 102, 103, 169, 13, 80, 99, 8, 27, 185, 109, 184, 83, 186, 106, 139, 70, 205, 204, 22, 87, 94, 140, 189, 79, 246, 97, 152, 72, 1, 33, 2, 113, 189, 124, 164, 245, 12, 96, 195, 100, 124, 115, 53, 29, 197, 235, 95, 104, 106, 186, 198, 59, 15, 56, 77, 132, 58, 107, 180, 80, 227, 138, 106, 255, 255, 255, 255, 1, 240, 35, 0, 0, 0, 0, 0, 0, 25, 118, 169, 20, 58, 175, 143, 10, 98, 89, 200, 2, 255, 208, 146, 136, 250, 224, 91, 40, 106, 79, 229, 68, 136, 172, 0, 0, 0]
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

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
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

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
        key_seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        
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

if __name__ == '__main__':
    unittest.main()
