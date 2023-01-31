import unittest
import demo_module

class TestAdd(unittest.TestCase):
    def test_add(self):
        self.assertEqual(demo_module.add(2, 3), 5, "Should be 5")

if __name__ == '__main__':
    unittest.main()