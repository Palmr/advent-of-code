use std::collections::HashMap;

/// --- Day 14: Docking Data ---
///
/// As your ferry approaches the sea port, the captain asks for your help again. The computer system that runs this port
/// isn't compatible with the docking program on the ferry, so the docking parameters aren't being correctly
/// initialized in the docking program's memory.
///
/// After a brief inspection, you discover that the sea port's computer system uses a strange bitmask system in its
/// initialization program. Although you don't have the correct decoder chip handy, you can emulate it in software!
///
/// The initialization program (your puzzle input) can either update the bitmask or write a value to memory. Values and
///  memory addresses are both 36-bit unsigned integers. For example, ignoring bitmasks for a moment, a line like
/// `mem[8] = 11` would write the value `11` to memory address `8`.
///
/// The bitmask is always given as a string of 36 bits, written with the most significant bit (representing 2^35) on the
/// left and the least significant bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied to
/// values immediately before they are written to memory: a `0` or `1` overwrites the corresponding bit in the value,
/// while an `X` leaves the bit in the value unchanged.
///
/// For example, consider the following program:
///
/// ```
/// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// mem[8] = 11
/// mem[7] = 101
/// mem[8] = 0
/// ```
///
/// This program starts by specifying a bitmask (`mask = ....`). The mask it specifies will overwrite two bits in every
/// written value: the `2s` bit is overwritten with `0`, and the `64s` bit is overwritten with `1`.
///
/// The program then attempts to write the value `11` to memory address `8`. By expanding everything out to individual
/// bits, the mask is applied as follows:
///
/// ```
/// value:  000000000000000000000000000000001011  (decimal 11)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001001001  (decimal 73)
/// ```
///
/// So, because of the mask, the value `73` is written to memory address `8` instead. Then, the program tries to write
/// `101` to address `7`:
///
/// ```
/// value:  000000000000000000000000000001100101  (decimal 101)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001100101  (decimal 101)
/// ```
///
/// This time, the mask has no effect, as the bits it overwrote were already the values the mask tried to set.
/// Finally, the program tries to write `0` to address `8`:
///
/// ```
/// value:  000000000000000000000000000000000000  (decimal 0)
/// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// result: 000000000000000000000000000001000000  (decimal 64)
/// ```
///
/// `64` is written to address `8` instead, overwriting the value that was there previously.
///
/// To initialize your ferry's docking program, you need the sum of all values left in memory after the initialization
/// program completes. (The entire 36-bit address space begins initialized to the value 0 at every address.)
///
/// In the above example, only two values in memory are not zero - `101` (at address `7`) and `64` (at address `8`) -
/// producing a sum of `165`.
///
/// Execute the initialization program. What is the sum of all values left in memory after it completes?
///
/// --- Part Two ---
///
/// For some reason, the sea port's computer system still can't communicate with your ferry's docking program.
/// It must be using version 2 of the decoder chip!
///
/// A version 2 decoder chip doesn't modify the values being written at all.
/// Instead, it acts as a memory address decoder.
/// Immediately before a value is written to memory, each bit in the bitmask modifies the corresponding bit of the
/// destination memory address in the following way:
///
///  - If the bitmask bit is 0, the corresponding memory address bit is unchanged.
///  - If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
///  - If the bitmask bit is X, the corresponding memory address bit is floating.
///
/// A floating bit is not connected to anything and instead fluctuates unpredictably.
/// In practice, this means the floating bits will take on all possible values, potentially causing many memory
/// addresses to be written all at once!
///
/// For example, consider the following program:
///
/// ```
/// mask = 000000000000000000000000000000X1001X
/// mem[42] = 100
/// mask = 00000000000000000000000000000000X0XX
/// mem[26] = 1
/// ```
///
/// When this program goes to write to memory address `42`, it first applies the bitmask:
///
/// ```
/// address: 000000000000000000000000000000101010  (decimal 42)
/// mask:    000000000000000000000000000000X1001X
/// result:  000000000000000000000000000000X1101X
/// ```
///
/// After applying the mask, four bits are overwritten, three of which are different, and two of which are floating.
/// Floating bits take on every possible combination of values;
/// with two floating bits, four actual memory addresses are written:
///
/// ```
/// 000000000000000000000000000000011010  (decimal 26)
/// 000000000000000000000000000000011011  (decimal 27)
/// 000000000000000000000000000000111010  (decimal 58)
/// 000000000000000000000000000000111011  (decimal 59)
/// ```
///
/// Next, the program is about to write to memory address 26 with a different bitmask:
///
/// ```
/// address: 000000000000000000000000000000011010  (decimal 26)
/// mask:    00000000000000000000000000000000X0XX
/// result:  00000000000000000000000000000001X0XX
/// ```
///
/// This results in an address with three floating bits, causing writes to eight memory addresses:
///
/// ```
/// 000000000000000000000000000000010000  (decimal 16)
/// 000000000000000000000000000000010001  (decimal 17)
/// 000000000000000000000000000000010010  (decimal 18)
/// 000000000000000000000000000000010011  (decimal 19)
/// 000000000000000000000000000000011000  (decimal 24)
/// 000000000000000000000000000000011001  (decimal 25)
/// 000000000000000000000000000000011010  (decimal 26)
/// 000000000000000000000000000000011011  (decimal 27)
/// ```
///
/// The entire 36-bit address space still begins initialized to the value 0 at every address, and you still need the
/// sum of all values left in memory at the end of the program. In this example, the sum is 208.
///
/// Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values
/// left in memory after it completes?
///
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MEM: Regex =
        Regex::new(r"^mem\[(?P<addr>[0-9]+)\] = (?P<value>[0-9]+)$").unwrap();
}
static MASK_36: u64 =
    0b0000_0000_0000_0000_0000_0000_0000_1111_1111_1111_1111_1111_1111_1111_1111_1111;

struct ValueMask {
    and: u64,
    or: u64,
}

impl ValueMask {
    fn new(mask: &str) -> ValueMask {
        let and_string = mask.replace("X", "1");
        let and_u64 = u64::from_str_radix(and_string.as_str(), 2).unwrap();
        let and_u36 = and_u64 & MASK_36;

        let or_string = mask.replace("X", "0");
        let or_u64 = u64::from_str_radix(or_string.as_str(), 2).unwrap();
        let or_u36 = or_u64 & MASK_36;

        ValueMask {
            and: and_u36,
            or: or_u36,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        value & self.and | self.or
    }
}

#[test]
fn test_value_mask() {
    let mask = ValueMask::new(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

    assert_eq!(73, mask.apply(11));
    assert_eq!(101, mask.apply(101));
    assert_eq!(64, mask.apply(0));
}

pub fn solve_part_one(input: &[String]) -> u64 {
    let mut current_mask: Option<ValueMask> = None;
    let mut memory: HashMap<usize, u64> = HashMap::new();

    input.iter().for_each(|l| match &l[0..3] {
        "mas" => {
            current_mask = Some(ValueMask::new(&l[7..]));
        }
        "mem" => {
            let matches = RE_MEM.captures(l).unwrap();
            let addr = matches
                .name("addr")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let value = matches
                .name("value")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            memory.insert(
                addr,
                current_mask.as_ref().map_or(value, |m| m.apply(value)),
            );
        }
        _ => panic!("Expecting `mask` and `mem` only"),
    });
    memory.values().into_iter().sum()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        165,
        solve_part_one(&[
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ])
    );
}

struct AddressMask {
    floating_bit_indicies: Vec<usize>,
    and: u64,
    or: u64,
}

impl AddressMask {
    fn new(mask: &str) -> AddressMask {
        let floating_bit_indicies = mask
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'X')
            .map(|(idx, _)| idx)
            .collect();

        // And mask to keep all bits but the floating
        let and_string = mask.replace("0", "1").replace("X", "0");
        let and_u64 = u64::from_str_radix(and_string.as_str(), 2).unwrap();
        let and_u36 = and_u64 & MASK_36;

        // Or mask to or mask ignoring floating
        let or_string = mask.replace("X", "0");
        let or_u64 = u64::from_str_radix(or_string.as_str(), 2).unwrap();
        let or_u36 = or_u64 & MASK_36;

        AddressMask {
            floating_bit_indicies,
            and: and_u36,
            or: or_u36,
        }
    }

    fn apply(&self, value: u64) -> Vec<u64> {
        // println!("value\t=\t{:036b}", value);
        // println!("and\t=\t{:036b}", self.and);
        // println!("or\t=\t{:036b}", self.or);

        let or_val = (value & self.and) | self.or;
        // println!("or_val\t=\t{:036b}", or_val);

        (0..2_usize.pow(self.floating_bit_indicies.len() as u32)).fold(Vec::new(), |mut acc, n| {
            let mut variant = or_val;
            for (bit_number, floating_bit_index) in self.floating_bit_indicies.iter().enumerate() {
                variant |= ((n as u64 >> bit_number) & 1) << (35 - *floating_bit_index);
            }
            // println!("Varia?nt[{}]:\t{:036b}", n, variant);
            acc.push(variant);
            acc
        })
    }
}

#[test]
fn test_address_mask() {
    let mask = AddressMask::new(&"000000000000000000000000000000X1001X");
    let mut addresses = mask.apply(42);
    addresses.sort_unstable();
    assert_eq!(vec![26, 27, 58, 59], addresses);

    let mask = AddressMask::new("00000000000000000000000000000000X0XX");
    let mut addresses = mask.apply(26);
    addresses.sort_unstable();
    assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], addresses);
}

pub fn solve_part_two(input: &[String]) -> u64 {
    let mut current_mask: Option<AddressMask> = None;
    let mut memory: HashMap<usize, u64> = HashMap::new();

    input.iter().for_each(|l| match &l[0..3] {
        "mas" => {
            current_mask = Some(AddressMask::new(&l[7..]));
        }
        "mem" => {
            let matches = RE_MEM.captures(l).unwrap();
            let addr = matches
                .name("addr")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let value = matches
                .name("value")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            for addr in current_mask.as_ref().map_or(vec![addr], |m| m.apply(addr)) {
                memory.insert(addr as usize, value);
            }
        }
        _ => panic!("Expecting `mask` and `mem` only"),
    });
    memory.values().into_iter().sum()
}

#[test]
fn examples_part_two() {
    assert_eq!(
        208,
        solve_part_two(&[
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ])
    );
}
