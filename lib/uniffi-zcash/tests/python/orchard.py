import unittest
from zcash import *

class Test(unittest.TestCase):
    def test_spending_key_conversions(self):
        key_bytes = [166, 3, 186, 151, 20, 139, 99, 33, 212, 134, 101, 192, 119, 208, 167, 21, 119, 228, 7, 152, 74, 140, 84, 209, 236, 235, 53, 57, 109, 65, 44, 178] 
        
        key = ZcashOrchardSpendingKey.from_bytes(key_bytes)

        self.assertEqual(key.to_bytes(), key_bytes)
    
    def test_spending_key_array_mismatch(self):
        key_bytes = [0, 1]

        with self.assertRaises(ZcashError.ArrayLengthMismatch):
            ZcashOrchardSpendingKey.from_bytes(key_bytes)

    def test_spending_key_from_zip32_seed(self):
        seed = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        coin_type = 234
        account = 2345

        key = ZcashOrchardSpendingKey.from_zip32_seed(seed, coin_type, account)

        expected_bytes = [23, 204, 133, 79, 99, 251, 110, 203, 15, 118, 24, 192, 12, 136, 237, 233, 13, 99, 222, 152, 174, 33, 68, 24, 46, 232, 217, 91, 241, 233, 151, 141]

        self.assertEqual(key.to_bytes(), expected_bytes)

if __name__ == '__main__':
    unittest.main()
