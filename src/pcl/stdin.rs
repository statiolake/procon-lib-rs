use std::io::prelude::*;
use std::io::stdin;
use std::str::FromStr;

pub fn read_line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn read<T: FromStr>() -> T {
    read_from(stdin())
}

pub fn read_token() -> String {
    read_token_from(stdin())
}

pub fn read_ascii() -> char {
    read_ascii_from(stdin())
}

pub fn read_from<R: Read, T: FromStr>(read: R) -> T {
    read_token_from(read)
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse a value"))
}

pub fn read_token_from<R: Read>(read: R) -> String {
    read.bytes()
        .flat_map(|x| x.map(|x| x as char))
        .skip_while(|x| x.is_whitespace())
        .take_while(|x| !x.is_whitespace())
        .collect()
}

pub fn read_ascii_from<R: Read>(read: R) -> char {
    read.bytes()
        .flat_map(|x| x.map(|x| x as char))
        .skip_while(|x| x.is_whitespace())
        .next()
        .expect("failed to get a next character")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn read() {
        let mut read = Cursor::new(b"123   423\r\n-234 hello\n43 42\n");
        assert_eq!(read_from::<_, i32>(&mut read), 123);
        assert_eq!(read_from::<_, u64>(&mut read), 423);
        assert_eq!(read_from::<_, i16>(&mut read), -234);
        assert_eq!(read_from::<_, String>(&mut read), "hello");
        assert_eq!(read_from::<_, String>(&mut read), "43");
        assert_eq!(read_from::<_, i32>(&mut read), 42);
        assert_eq!(read_from::<_, String>(&mut read), "");
    }

    #[test]
    fn read_mixed() {
        let mut read = Cursor::new(b"123 4  56\r\n789");
        assert_eq!(read_from::<_, i32>(&mut read), 123);
        assert_eq!(read_ascii_from(&mut read), '4');
        assert_eq!(read_ascii_from(&mut read), '5');
        assert_eq!(read_token_from(&mut read), "6");
        assert_eq!(read_ascii_from(&mut read), '7');
        assert_eq!(read_from::<_, u8>(&mut read), 89);
    }

    #[test]
    #[should_panic]
    fn read_unmatch_sign() {
        read_from::<_, u32>(&b"-123"[..]);
    }

    #[test]
    #[should_panic]
    fn read_ended() {
        read_from::<_, i32>(&b""[..]);
    }

    #[test]
    fn read_token() {
        let mut read = Cursor::new(b"123 abcdef  \tghi\r\n");
        assert_eq!(read_token_from(&mut read), "123");
        assert_eq!(read_token_from(&mut read), "abcdef");
        assert_eq!(read_token_from(&mut read), "ghi");
    }

    #[test]
    fn read_ascii() {
        let mut read = Cursor::new("hello \t w orld");
        assert_eq!(read_ascii_from(&mut read), 'h');
        assert_eq!(read_ascii_from(&mut read), 'e');
        assert_eq!(read_ascii_from(&mut read), 'l');
        assert_eq!(read_ascii_from(&mut read), 'l');
        assert_eq!(read_ascii_from(&mut read), 'o');
        assert_eq!(read_ascii_from(&mut read), 'w');
        assert_eq!(read_ascii_from(&mut read), 'o');
        assert_eq!(read_ascii_from(&mut read), 'r');
        assert_eq!(read_ascii_from(&mut read), 'l');
        assert_eq!(read_ascii_from(&mut read), 'd');
    }

    #[test]
    #[should_panic]
    fn read_ascii_ended() {
        read_ascii_from(&b""[..]);
    }
}
