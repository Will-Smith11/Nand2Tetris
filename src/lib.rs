use core::fmt;
use std::fmt::Binary;
struct CInst
{
    dest: Option<String>,
    comp: String,
    jump: Option<String>,
    line_number: u128,
}

struct AInst
{
    addr: String
}

impl Binary for AInst
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.addr.parse::<u128>().unwrap();

        fmt::Binary::fmt(&val, f)
    } 
}
