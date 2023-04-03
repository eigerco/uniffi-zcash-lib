import uniffi.zcash.*



impl ZcashExpandedSpendingKey {
    pub fn from_spending_key(sk: Vec<u8>) -> Self {
        ExpandedSpendingKey::from_spending_key(&sk).into()
    }

    pub fn from_bytes(b: Vec<u8>) -> ZcashResult<Self> {
        ExpandedSpendingKey::from_bytes(&b)
            .map(From::from)
            .map_err(From::from)
    }

    pub fn proof_generation_key(&self) -> Arc<ZcashProofGenerationKey> {
        Arc::new(self.0.proof_generation_key().into())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

fun testFromExpandedSpendingKey() {
	val supp = TestSupport.fromCsvFile()

	// is this the correct spending key?
	val skBytes = supp.getAsByteArray("extended_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromSpendingKey(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromExpandedSpendingKey()

fun testFromBytes() {
	val supp = TestSupport.fromCsvFile()

	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	assert(spendingKey.toBytes() == skBytes)
}
testFromBytes()

fun testProofGenerationKey() {
	val supp = TestSupport.fromCsvFile()

	val skBytes = supp.getAsByteArray("expanded_spending_key")

	val spendingKey = ZcashExpandedSpendingKey.fromBytes(skBytes)

	val proofGenKey = spendingKey.proofGenerationKey()

	// TODO finish this, but how?
	assert(false)

}
testProofGenerationKey()
