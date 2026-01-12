import unittest


class TestSolution(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        """Read the test input file."""
        with open("../test_input.txt", "r", encoding="utf-8") as f:
            cls.test_input = f.read()

    def test_solve_example_case(self):
        """Test that the example input gives the correct result."""
        want = 5031
        got = "The test case is not implemented for this one"
        self.assertEqual(want, got)


if __name__ == "__main__":
    unittest.main()
