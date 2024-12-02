package main

import (
	"fmt"
  "math"
)

// Problem definition:
// distance = velocity * time_moving
// velocity = time_held
// race_time = time_moving + time_held

// re-arrange to get:
// time_held^2 - race_time * time_held + distance = 0
// ... and now things look a lot easier!

func is_integer_value(value float64) bool {
  return math.Mod(value, 1.0) == 0
}

func calculate_margin_of_error(race_time float64, race_distance float64) uint64 {
  lower_root := (race_time - math.Sqrt(race_time * race_time - 4 * race_distance)) / 2
  upper_root := (race_time + math.Sqrt(race_time * race_time - 4 * race_distance)) / 2

  var lowest_winning_value uint64
  var highest_winning_value uint64

  // Since the root value corresponds to matching the record (not beating it) we need to
  // round up the lower root and round down the upper root
  if is_integer_value(lower_root) {
    lowest_winning_value = uint64(lower_root + 1)
  } else {
    lowest_winning_value = uint64(math.Ceil(lower_root))
  }

  if is_integer_value(upper_root) {
    highest_winning_value = uint64(upper_root - 1)
  } else {
    highest_winning_value = uint64(math.Floor(upper_root))
  }

  return highest_winning_value - lowest_winning_value + 1
}

func main() {
  // The file is small so let's just hardcode the values
  margin_of_error := calculate_margin_of_error(49787980, 298118510661181)

  fmt.Printf("The margin of error is: %d\n", margin_of_error)
}
