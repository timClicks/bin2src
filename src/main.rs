// SPDX-License-Identifier: Apache-2.0

use std::io;
use std::io::{Read};

const MAX_LINEWIDTH: usize = 90;
const NEWLINE: &'static str = "\n";
const LINE_START: &'static str = "    ";
const LINE_END: &'static str = "\\";
const BYTE_DELIMITER: &'static str = "";
const PROLOGUE: &'static str = "const DATA: &[u8] = b\"";
const EPILOGUE: &'static str = "\";";

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut current_line_length: usize = PROLOGUE.len();
    
    print!("{}", PROLOGUE);
    for byte in stdin.bytes() {
        let term = format!("\\x{:02X}{}", byte.unwrap(), BYTE_DELIMITER);
        if current_line_length + term.len() > MAX_LINEWIDTH-2 {
            print!("{}{}{}", LINE_END, NEWLINE, LINE_START);
            current_line_length = LINE_START.len();
        }

        print!("{}", term);
        current_line_length += term.len();
    }
    println!("{}", EPILOGUE);

    Ok(())
}
