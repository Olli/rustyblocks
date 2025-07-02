//use super::Block;

#[derive(Debug)]
pub struct Block<'a> {
    pub icon: &'a str,
    pub command: &'a str,
    pub interval: u8,
    pub signal: u8,
}

pub const BLOCKS: &[Block] = &[
    /*Icon*/ /*Command*/ /*Update Interval*/ /*Update Signal*/
    Block {
        icon: "",
        command: "setxkbmap -query | awk '/^layout:/ { print  $2 }'",
        interval: 2,
        signal: 0,
    },
    Block {
        icon: " ",
        command: "sensors  | awk '/^Package id 0/ {print $4}'",
        interval: 30,
        signal: 0,
    },
    Block {
        icon: " ",
        command: "echo \"$(grep -o '^[^ ]*' /proc/loadavg)\"",
        interval: 5,
        signal: 0,
    },
    Block {
        icon: " ",
        command: "free -h | awk '/^Speicher/ { print $3 }' | sed s/i//g",
        interval: 5,
        signal: 0,
    },
    Block {
        icon: "󱑆 ",
        command: "date '+%a %b %d %H:%M%p'",
        interval: 5,
        signal: 0,
    },
];

// sets delimiter between status commands. NULL character ('\0') means no
// delimiter.
pub const SEPARATOR: &str = " | ";
pub const SEPARATOR_LEN: u8 = 5;
