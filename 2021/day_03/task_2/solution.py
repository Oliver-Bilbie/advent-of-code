def bin_to_int(binary_num: list[int]) -> int:
    result = 0
    for bit in binary_num:
        result *= 2
        result += bit
    return result


def result(input_file: str) -> int:
    report = [[int(c) for c in line] for line in input_file.splitlines()]

    o2_gen = report.copy()
    bit = 0
    while len(o2_gen) > 1:
        bit_is_on = sum((l[bit] for l in o2_gen)) >= len(o2_gen) / 2
        if bit_is_on:
            o2_gen = [v for v in o2_gen if v[bit] == 1]
        else:
            o2_gen = [v for v in o2_gen if v[bit] == 0]
        bit += 1
    o2_gen_value = bin_to_int(o2_gen[0])

    co2_scrub = report
    bit = 0
    while len(co2_scrub) > 1:
        bit_is_on = sum((l[bit] for l in co2_scrub)) < len(co2_scrub) / 2
        if bit_is_on:
            co2_scrub = [v for v in co2_scrub if v[bit] == 1]
        else:
            co2_scrub = [v for v in co2_scrub if v[bit] == 0]
        bit += 1
    co2_scrub_value = bin_to_int(co2_scrub[0])

    return o2_gen_value * co2_scrub_value


def solve(input_file: str) -> str:
    return f"The life support rating of the submarine is {result(input_file)}"
