#![no_main]

use RR::syntax::parse::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(src) = std::str::from_utf8(data) else {
        return;
    };
    let mut parser = Parser::new(src);
    let _ = parser.parse_program();
});

