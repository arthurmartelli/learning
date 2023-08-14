#![allow(unused)]

use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
