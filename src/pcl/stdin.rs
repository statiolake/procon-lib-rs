use std::io::prelude::*;
use std::io::stdin;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    read_read(stdin())
}

pub fn read_line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn read_read<R: Read, T: FromStr>(read: R) -> T {
    let s: Vec<u8> = read
        .bytes()
        .take_while(|x| {
            // continue if Err is returned to propagate an error as a panic
            x.as_ref()
                .map(|x| ![b' ', b'\r', b'\t', b'\n'].contains(&x))
                .unwrap_or(true)
        })
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| panic!("failed to read bytes"));

    String::from_utf8_lossy(&s)
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse a value"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read() {
        let mut read = Cursor::new(b"123 423\n-234 hello\n43 42\n");

        let x: i32 = read_read(&mut read);
        assert_eq!(x, 123);

        let x: usize = read_read(&mut read);
        assert_eq!(x, 423);

        let x: i16 = read_read(&mut read);
        assert_eq!(x, -234);

        let x: String = read_read(&mut read);
        assert_eq!(x, "hello");
    }
}
