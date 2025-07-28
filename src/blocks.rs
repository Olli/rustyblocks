#[derive(Debug)]
pub struct Block<'a> {
    pub icon: &'a str,
    pub command: &'a str,
    pub interval: u32,
    pub signal: u8,
}

pub const BLOCKS: &[Block] = &[
    /*Icon*/ /*Command*/ /*Update Interval*/ /*Update Signal*/
    Block {
        icon: "",
        command: "curl wttr.in/berlin?format=4",
        interval: 3600,
        signal: 0,
    },
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

// sets delimiter between status commands.
pub const SEPARATOR: &str = " | ";
