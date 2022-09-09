struct U32Or<const T: u32>(u32);

impl<const T: u32> Default for U32Or<T> {
    fn default() -> Self {
        Self(T)
    }
}

struct BoolOr<const T: bool>(bool);

impl<const T: bool> Default for BoolOr<T> {
    fn default() -> Self {
        Self(T)
    }
}

struct CharOr<const T: char>(char);

impl<const T: char> Default for CharOr<T> {
    fn default() -> Self {
        Self(T)
    }
}