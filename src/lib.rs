use std::mem::size_of;

pub struct GermanString([u8; size_of::<u128>()]);

impl GermanString {
    pub fn new() -> Self {
        Self([0; size_of::<u128>()])
    }

    #[inline]
    pub fn len(&self) -> usize {
        u32::from_ne_bytes(self.0[..size_of::<u32>()].try_into().unwrap()) as usize
    }

    pub fn push_str(&mut self, s: &str) {
        let new_len = s.len() + self.len();
        if new_len <= 12 {
            let offset = size_of::<u32>() + self.len();
            self.0[offset..offset + s.len()].copy_from_slice(s.as_bytes());
            self.0[..size_of::<u32>()].copy_from_slice(&(new_len as u32).to_ne_bytes());
        } else {
            todo!()
        }
    }
}

impl AsRef<str> for GermanString {
    fn as_ref(&self) -> &str {
        if self.len() <= 12 {
            let offset = size_of::<u32>();
            unsafe {
                std::str::from_utf8_unchecked(&self.0[offset..offset + self.len()])
            }
        } else {
            todo!()
        }
    }
}

impl PartialEq for GermanString {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        } else {
            self.0[size_of::<u32>()..size_of::<u32>() + self.len()] == other.0[size_of::<u32>()..size_of::<u32>() + other.len()]

        }
    }
}

impl Eq for GermanString {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        let mut s = GermanString::new();
        s.push_str("hello");
        assert_eq!(s.as_ref(), "hello");
    }
}

