import unittest
import solution


class TestSolution(unittest.TestCase):
    def test_solve_example_case_1(self):
        """Test that the first example input gives the correct result."""
        example_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
        want = 19
        got = solution.result(example_input)
        self.assertEqual(want, got)

    def test_solve_example_case_2(self):
        """Test that the second example input gives the correct result."""
        example_input = "bvwbjplbgvbhsrlpgdmjqwftvncz"
        want = 23
        got = solution.result(example_input)
        self.assertEqual(want, got)

    def test_solve_example_case_3(self):
        """Test that the third example input gives the correct result."""
        example_input = "nppdvjthqldpwncqszvftbrmjlhg"
        want = 23
        got = solution.result(example_input)
        self.assertEqual(want, got)

    def test_solve_example_case_4(self):
        """Test that the fourth example input gives the correct result."""
        example_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        want = 29
        got = solution.result(example_input)
        self.assertEqual(want, got)

    def test_solve_example_case_5(self):
        """Test that the fifth example input gives the correct result."""
        example_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        want = 26
        got = solution.result(example_input)
        self.assertEqual(want, got)


if __name__ == "__main__":
    unittest.main()
