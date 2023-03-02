use std::io;

pub(crate) fn read_line() -> io::Result<String> {
    let mut line = String::new();

    io::stdin().read_line(&mut line)?;

    Ok(line)
}

pub(crate) mod macros {
    macro_rules! d_println {
        ($format:literal, $tt:tt) => {
            #[cfg(debug_assertions)]
            println!(concat!("[DEBUG]: ", $format), $tt);
        };
    }

    pub(crate) use d_println;
}
