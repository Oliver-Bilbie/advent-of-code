import unittest
import solution


class TestSolution(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        """Read the test input file."""
        with open("../test_input.txt", "r", encoding="utf-8") as f:
            cls.test_input = f.read()
        with open("../test_input_2.txt", "r", encoding="utf-8") as f:
            cls.test_input_2 = f.read()

    def test_solve_example_case(self):
        """Test that the example input gives the correct result."""
        want = 1
        got = solution.result(self.test_input)
        self.assertEqual(want, got)

    def test_solve_example_case_2(self):
        """Test that the new example input gives the correct result."""
        want = 36
        got = solution.result(self.test_input_2)
        self.assertEqual(want, got)


if __name__ == "__main__":
    unittest.main()
