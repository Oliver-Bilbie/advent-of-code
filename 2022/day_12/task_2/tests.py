import unittest
import solution


class TestSolution(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        """Read the test input file."""
        with open("../test_input.txt", "r", encoding="utf-8") as f:
            cls.test_input = f.read()

    def test_solve_example_case(self):
        """Test that the example input gives the correct result."""
        want = 29
        got = solution.result(self.test_input)
        self.assertEqual(want, got)


if __name__ == "__main__":
    unittest.main()
