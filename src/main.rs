use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
extern crate trees;
use trees::{tr, Tree};

// Advent of Code 2019: https://adventofcode.com/2019
fn main() {
  let data1 = [
    112908, 61769, 65967, 51494, 99689, 114098, 135346, 59561, 147324, 50465, 117491, 77845, 91959,
    59847, 84013, 85763, 62121, 58965, 89809, 97870, 77696, 70218, 118404, 83505, 141729, 61534,
    101073, 131358, 76104, 120836, 109789, 96510, 65406, 117906, 74921, 95412, 99875, 134298,
    105235, 82802, 103392, 81906, 133786, 140681, 109004, 148434, 92333, 83848, 98297, 95728,
    138202, 79312, 55633, 138461, 50293, 141922, 140303, 66542, 50054, 99076, 143201, 66702,
    133583, 70308, 146824, 95606, 63054, 129272, 133051, 58626, 119896, 66265, 99925, 131752,
    74669, 148387, 132124, 107188, 55535, 145004, 78122, 50885, 70325, 100859, 86484, 88795,
    148164, 64473, 143089, 121023, 52904, 120927, 87164, 133709, 89427, 105350, 106378, 98492,
    78394, 145200,
  ];
  let data2: [isize; 121] = [
    1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 2, 19, 6, 23, 2, 13, 23, 27, 1,
    9, 27, 31, 2, 31, 9, 35, 1, 6, 35, 39, 2, 10, 39, 43, 1, 5, 43, 47, 1, 5, 47, 51, 2, 51, 6, 55,
    2, 10, 55, 59, 1, 59, 9, 63, 2, 13, 63, 67, 1, 10, 67, 71, 1, 71, 5, 75, 1, 75, 6, 79, 1, 10,
    79, 83, 1, 5, 83, 87, 1, 5, 87, 91, 2, 91, 6, 95, 2, 6, 95, 99, 2, 10, 99, 103, 1, 103, 5, 107,
    1, 2, 107, 111, 1, 6, 111, 0, 99, 2, 14, 0, 0,
  ];
  let data3_line1: Vec<&str> = "R1003,D138,L341,U798,L922,U153,R721,D177,L297,D559,L414,U470,L589,D179,L267,D954,R739,D414,L865,U688,R541,U242,R32,D607,L480,D401,L521,U727,L295,D154,R905,D54,L353,U840,L187,U942,R313,D143,R927,D962,R739,U152,R6,D9,L807,D67,R425,D235,L598,D107,L838,D522,L882,U780,L942,D29,R933,U129,L556,D11,L859,D455,L156,U673,L54,D141,R862,U88,R362,U742,L511,D408,R825,U622,R650,D393,L882,D969,R866,D232,L423,U371,L744,U35,L196,D189,R803,U663,R41,U741,R742,U929,L311,U30,R357,D776,L929,U85,R415,U540,R921,U599,R651,U79,R608,D620,L978,D92,L491,D310,L830,U656,R244,U72,L35,U768,R666,U356,R82,U596,L798,D455,L280,D626,R586,U668,R331,D245,L140,U3,R283,U813,R620,U975,L795,U477,L100,D94,R353,D732,R694,U702,L305,U497,R900,U810,L412,D954,R584,D444,L531,D875,R49,D328,L955,U227,L370,D548,L351,U571,R373,U743,R105,D226,L755,U325,R496,D960,L415,U262,R197,D508,R725,U930,L722,D162,L996,D610,R346,U680,L75,U211,R953,U147,R114,D48,L305,D284,L630,U575,R142,D518,R704,D820,L617,D118,R67,D674,L90,D916,L483,D598,L424,U92,R188,U413,L702,D262,R720,D995,L759,D732,L259,D814,L342,U642,L875,U726,R265,D143,R754,D235,L535,U1,R211,D720,R943,D726,L398,U636,R994,U653,L401,U877,R577,D460,L730,U889,R166,D641,L693,U490,L78,D80,R535,U551,L866,U283,L336,U586,L913,U474,R158,D220,R278,U11,R421,D661,R719,D696,R188,D735,L799,U391,R331,U581,R689,D82,R375,D125,R613,D705,L927,U18,R399,D352,L411,D777,L733,D884,R791,U973,R772,D878,R327,U215,L298,D360,R426,D872,L99,U78,L745,U59,L641,U73,L294,D247,R944,U512,L396".split(',').collect();
  let data3_line2: Vec<&str> = "L1004,D252,L909,D935,R918,D981,L251,U486,R266,U613,L546,D815,L789,D692,L550,U633,R485,U955,R693,D784,R974,U529,R926,U550,L742,U88,R647,D572,R832,D345,R98,D122,R634,U943,L956,U551,R295,U122,L575,U378,R652,U97,R129,D872,R275,D492,L530,D328,R761,U738,R836,U884,R636,U776,L951,D977,R980,U526,L824,U125,R778,D818,R281,U929,R907,U234,L359,D521,R294,U137,L607,U421,L7,U582,R194,U979,L941,D999,R442,D330,L656,U410,R753,U704,R834,D61,R775,U750,R891,D989,R856,D944,R526,D44,R227,U938,R130,D280,L721,D171,R763,D677,L643,U931,L489,U250,L779,U552,R796,U220,R465,U700,L459,U766,R501,D16,R555,U257,R122,U452,L197,U905,L486,D726,L551,U487,R785,U470,L879,U149,L978,D708,R18,U211,L652,D141,L99,D190,L982,U556,R861,U745,L786,U674,R706,U986,R554,D39,R881,D626,R885,U907,R196,U532,L297,U232,L508,U283,L236,U613,L551,U647,R679,U760,L435,D475,R916,U669,R788,U922,R107,D503,R687,D282,L940,U835,L226,U421,L64,U245,R977,D958,L866,D328,R215,D532,R350,D199,R872,U373,R415,U463,L132,U225,L144,U786,R658,D535,R263,U263,R48,D420,L407,D177,L496,U521,R47,D356,L503,D557,R220,D879,L12,U853,R265,U983,L221,U235,R46,D906,L271,U448,L464,U258,R952,D976,L949,D526,L458,D753,L408,U222,R256,U885,R986,U622,R503,D5,L659,D553,R311,U783,L541,U17,R267,U767,L423,D501,R357,D160,L316,D912,R303,U648,L182,U342,L185,U743,L559,U816,R24,D203,R608,D370,R25,U883,L72,D816,L877,U990,R49,U331,L482,U37,L585,D327,R268,D106,L224,U401,L203,D734,L695,U910,L417,U105,R135,U876,L194,U723,L282,D966,R246,U447,R966,U346,L636,D9,L480,D35,R96".split(',').collect();
  let data4_start: Vec<u8> = vec![1, 0, 9, 1, 6, 5];
  let data4_endplusone: Vec<u8> = vec![5, 7, 6, 7, 2, 4]; // one past
  let data5: [isize; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 67, 92, 225, 1101, 14, 84, 225, 1002,
    217, 69, 224, 101, -5175, 224, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223,
    223, 1, 214, 95, 224, 101, -127, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 223,
    224, 223, 1101, 8, 41, 225, 2, 17, 91, 224, 1001, 224, -518, 224, 4, 224, 1002, 223, 8, 223,
    101, 2, 224, 224, 1, 223, 224, 223, 1101, 37, 27, 225, 1101, 61, 11, 225, 101, 44, 66, 224,
    101, -85, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223, 223, 1102, 7, 32,
    224, 101, -224, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 224, 223, 223, 1001,
    14, 82, 224, 101, -174, 224, 224, 4, 224, 102, 8, 223, 223, 101, 7, 224, 224, 1, 223, 224, 223,
    102, 65, 210, 224, 101, -5525, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224,
    223, 223, 1101, 81, 9, 224, 101, -90, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1,
    224, 223, 223, 1101, 71, 85, 225, 1102, 61, 66, 225, 1102, 75, 53, 225, 4, 223, 99, 0, 0, 0,
    677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005,
    227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006,
    0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101,
    294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101,
    314, 0, 0, 106, 0, 0, 1105, 1, 99999, 8, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 1001,
    223, 1, 223, 1108, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 1007,
    226, 677, 224, 102, 2, 223, 223, 1005, 224, 359, 101, 1, 223, 223, 1007, 677, 677, 224, 1002,
    223, 2, 223, 1006, 224, 374, 101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1005,
    224, 389, 1001, 223, 1, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 404, 101, 1, 223,
    223, 1108, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 419, 101, 1, 223, 223, 1008, 677, 677,
    224, 102, 2, 223, 223, 1005, 224, 434, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223,
    1005, 224, 449, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 464, 1001,
    223, 1, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 479, 1001, 223, 1, 223, 107,
    677, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1008, 226, 677, 224, 102,
    2, 223, 223, 1006, 224, 509, 1001, 223, 1, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1005,
    224, 524, 101, 1, 223, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 539, 1001, 223,
    1, 223, 107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 554, 101, 1, 223, 223, 108, 677, 677,
    224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 7, 226, 677, 224, 102, 2, 223, 223,
    1006, 224, 584, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 599, 101, 1,
    223, 223, 1107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 614, 101, 1, 223, 223, 8, 226,
    677, 224, 102, 2, 223, 223, 1005, 224, 629, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2,
    223, 1006, 224, 644, 1001, 223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 659,
    101, 1, 223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 674, 101, 1, 223, 223, 4,
    223, 99, 226,
  ];

  let mut start = Instant::now();
  println!("1.1: {}", day1_1(&data1)); // Expected: 3275518
  println!("1.2: {}", day1_2(&data1)); // Expected: 4910404
  println!("2.1: {}", day2_1(&data2)); // Expected: 3716293
  println!("2.2: {}", day2_2(&data2, 19690720)); // Expected: 6429
  output_time_elapsed(&mut start);
  let result3 = day3(data3_line1, data3_line2); // Expected: 446, 9006
  println!("3.1: {}\n3.2: {}", result3.0, result3.1);
  output_time_elapsed(&mut start);
  println!("4.1: {}", day4_1(data4_start.to_vec(), &data4_endplusone)); // Expected: 2814
  println!("4.2: {}", day4_2(data4_start.to_vec(), &data4_endplusone)); // Expected: 1991
  output_time_elapsed(&mut start);
  println!("5.1: {}", intcode_computer(&mut data5.to_vec(), 1, false)); // Expected: 5074395
  println!("5.2: {}", intcode_computer(&mut data5.to_vec(), 5, false)); // Expected: 8346937
  output_time_elapsed(&mut start);
  day6();
}

// Output elapsed time to the console
fn output_time_elapsed(start: &mut Instant) {
  println!("  Elapsed: {}", start.elapsed().as_secs_f32());
  *start = Instant::now();
}

// This IntCo Computer is built over several problems by adding new OpCos
// Opcode 1: adds together numbers read from two positions and stores the result in a third position.
// Opcode 2: multiplies together numbers read from two positions and stores the result in a third position.
// Opcode 3: Single input saved to location
// Opcode 4: Output the value of the parameter
// Opcode 99: halt program
// Parameter Modes:
//  0 - position (in the instruction sequence)
//  1 - value
//  Right to left, so 1002 is an Mult of a position, value, position (leading zero absent)
fn intcode_computer(lst: &mut Vec<isize>, input: isize, verbose: bool) -> isize {
  let mut output: isize = -1;
  if verbose {
    println!("|--------------------------------------|");
    println!("|Welcome to INTCODE v42.34.a:");
  }
  let mut i = 0;
  while i < lst.len() {
    // Extract the right to chars to opco, then save the rest for params
    let opco = lst[i] % 100;
    let mut params = lst[i] / 100;
    match opco {
      1 => {
        // Add
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        let param3 = lst[i + 3] as usize;
        lst[param3] = param1 + param2;
        i += 4;
      }
      2 => {
        // Mult
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        let param3 = lst[i + 3] as usize;
        lst[param3] = param1 * param2;
        i += 4;
      }
      3 => {
        // Input
        let param1 = lst[i + 1] as usize;
        lst[param1] = input;
        i += 2;
        if verbose {
          println!("|Input: {}", input);
        }
      }
      4 => {
        // Output
        let param1 = get_param(&lst, i + 1, &mut params);
        output = param1;
        i += 2;
        if verbose {
          println!("|> {}", param1);
        }
      }
      5 => {
        // Jump if true
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        i = if param1 != 0 { param2 as usize } else { i + 3 }
      }
      6 => {
        // Jump if false
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        i = if param1 == 0 { param2 as usize } else { i + 3 }
      }
      7 => {
        // Less Than
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        let param3 = lst[i + 3] as usize;
        lst[param3] = if param1 < param2 { 1 } else { 0 };
        i += 4;
      }
      8 => {
        // Equals
        let param1 = get_param(&lst, i + 1, &mut params);
        let param2 = get_param(&lst, i + 2, &mut params);
        let param3 = lst[i + 3] as usize;
        lst[param3] = if param1 == param2 { 1 } else { 0 };
        i += 4;
      }
      99 => {
        if verbose {
          println!("|Goodbye!");
        }
        break;
      }
      _ => {
        println!("|Unexpected OpCode at {}: {}", i, lst[i]);
        break;
      }
    }
  }
  if verbose {
    println!("|--------------------------------------|");
  }
  output
}

// Gets a parameter based on byval/byref
fn get_param(lst: &Vec<isize>, i: usize, params: &mut isize) -> isize {
  let byval = *params % 10 == 1;
  *params = *params / 10;
  return if byval { lst[i] } else { lst[lst[i] as usize] };
}

// Fuel required to launch a given module is based on its mass:alloc
//   Take its mass, divide by three, round down, and subtract 2.
fn day1_1(lst: &[usize]) -> usize {
  let mut fuel = 0;

  for x in lst.iter() {
    fuel += (x / 3) - 2;
  }
  fuel
}

// Recrusively add fuel for the fuel, ignoring negatives.
// So, the total fuel required for a module of mass 1969 is
//   654 + 216 + 70 + 21 + 5 = 966.
fn day1_2(lst: &[usize]) -> i64 {
  let mut fuel: i64 = 0; // Need i64 due to overflow

  for x in lst.iter() {
    let mut y: i64 = *x as i64; // had to dereference to cast
    loop {
      y = (y / 3) - 2;
      if y > 0 {
        fuel = fuel + y;
      } else {
        break;
      }
    }
  }
  fuel
}

// Implement simple OpCo program with instructions 1, 2, 99.  Return first element.
fn day2_1(lst: &[isize]) -> isize {
  let mut tmp = lst.to_vec();
  intcode_computer(&mut tmp, 0, false);
  tmp[0]
}

// Determine what pair of inputs produces the target
// [0]: target output, [1]: x, [2]: y
// Answer = 100x + y
fn day2_2(lst: &[isize], target: isize) -> isize {
  for x in 1..100 {
    for y in 1..100 {
      let mut tmp = lst.to_vec();
      tmp[1] = x;
      tmp[2] = y;
      intcode_computer(&mut tmp, 0, false);

      if tmp[0] == target {
        return 100 * x + y;
      }
    }
  }
  0
}

// Given two lines that start at 0,0: R8,U5,L5,D3; U7,R6,D4,L4
//   .+-----+...
//   .|.....|...
//   .|..+--X-+.
//   .|..|..|.|.
//   .|.-X--+.|.
//   .|..|....|.
//   .|.......|.
//   .o-------+.
// Find the distance to the nearest intersection: (3,3) => 3 + 3 = 6
fn day3(line1: Vec<&str>, line2: Vec<&str>) -> (i32, i32) {
  // Use a HashSet for line points to allow for an intersection command
  // HashSet = single item, HashMap = key/value pair
  let mut points1: HashSet<(i32, i32)> = HashSet::new();
  let mut points2: HashSet<(i32, i32)> = HashSet::new();
  let mut point_distance_1: HashMap<(i32, i32), i32> = HashMap::new();
  let mut point_distance_2: HashMap<(i32, i32), i32> = HashMap::new();

  day3_add_points(line1, &mut points1, &mut point_distance_1);
  day3_add_points(line2, &mut points2, &mut point_distance_2);

  // Find nearest for 3.1, smallest steps for 3.2
  let hits = points1.intersection(&points2);
  let mut min1 = i32::MAX;
  let mut min2 = i32::MAX;
  for h in hits {
    // The lines do go to negative coordinates, so just use absolute value
    let z1 = h.0.abs() + h.1.abs();
    min1 = cmp::min(min1, z1);

    let z2 = point_distance_1.get(&h).unwrap() + point_distance_2.get(&h).unwrap();
    min2 = cmp::min(min2, z2);
  }
  return (min1, min2);
}

// Adds points from a list of segments
fn day3_add_points(
  line: Vec<&str>,
  points: &mut HashSet<(i32, i32)>,
  point_distance: &mut HashMap<(i32, i32), i32>,
) {
  // These will walk the line
  let mut x = 0;
  let mut y = 0;
  let mut steps = 0;

  // We don't add the start of each line (0,0) because that would just be another intersection we don't need

  // Walk through each command
  for seg in &line {
    let mut dx = 0;
    let mut dy = 0;

    // Determine the direction by looking at the first character
    match seg.chars().next().unwrap() as char {
      'R' => dx = 1,
      'L' => dx = -1,
      'U' => dy = 1,
      'D' => dy = -1,
      _ => panic!("Unexpected direction (allowed: RLUD), may be a data issue"),
    }
    // Get the distance using a range
    let n = seg[1..].parse::<i32>().unwrap();

    // Add each point along the line, zero based because "n" does not go in the loop
    for _ in 0..n {
      steps += 1;
      x += dx;
      y += dy;
      points.insert((x, y));
      point_distance.insert((x, y), steps);
    }
  }
}

// How many different passwords
//   Range 109165 - 576723
//   2 adjacent digits are the same (122345)
//   Digits always increase or stay same
fn day4_1(mut v: Vec<u8>, end_plus_one: &Vec<u8>) -> u32 {
  let mut count = 0;

  while v != *end_plus_one {
    if (v[0] == v[1] || v[1] == v[2] || v[2] == v[3] || v[3] == v[4] || v[4] == v[5])
      && (v[0] <= v[1] && v[1] <= v[2] && v[2] <= v[3] && v[3] <= v[4] && v[4] <= v[5])
    {
      count += 1;
    }
    day4_1_increment(&mut v);
  }
  count
}

// Increments the contents of a slice as if it was one big number
fn day4_1_increment(v: &mut Vec<u8>) {
  for i in (0..v.len()).rev() {
    v[i] += 1;
    if v[i] == 10 {
      v[i] = 0;
    } else {
      break;
    }
  }
}

// How many different passwords
//   Also: 2 adj digits not part of a LARGER group of matching digits (one good set is needed)
//     bad: 123444  (44 part of 444)
//     good: 111122 (11 not part of 22)
fn day4_2(mut v: Vec<u8>, end_plus_one: &Vec<u8>) -> u32 {
  let mut count = 0;

  while v != *end_plus_one {
    if (v[0] == v[1] || v[1] == v[2] || v[2] == v[3] || v[3] == v[4] || v[4] == v[5])
      && (v[0] <= v[1] && v[1] <= v[2] && v[2] <= v[3] && v[3] <= v[4] && v[4] <= v[5])
    {
      for i in (0..10).rev() {
        if day4_2_dup_digit_count(i, &v) == 2 {
          count += 1;
          break;
        }
      }
    }
    day4_1_increment(&mut v);
  }
  count
}

// Count maximum times a number appears in a row in a vector
fn day4_2_dup_digit_count(val: u8, v: &[u8]) -> u8 {
  let mut counter = 0;
  let mut final_counter = 0;

  for i in 0..6 {
    if val == v[i] {
      counter += 1;
    } else {
      final_counter = cmp::max(final_counter, counter);
      counter = 0;
    }
  }
  cmp::max(final_counter, counter)
}

fn day6() {
  let mut tree: Tree<String> = tr("MQD".to_string());
  let mut root = tree.root_mut();
  root.push_back(tr("G37".to_string()));
  root.push_back(tr("what".to_string()));
  println!("{}", tree.to_string());
  //for n in tree.n
}
