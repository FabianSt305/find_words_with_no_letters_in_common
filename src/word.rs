/// Encodes which letters have been seen (0 = unseen, 1 = seen).
///
/// Bit layout:
/// `0000 00zy xwvu tsrq ponm lkji hgfe dcba`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Letters(u32);

impl Letters {
    pub fn new() -> Self {
        Self { 0: 0 }
    }

    pub fn shares_letters(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

impl From<u32> for Letters {
    fn from(x: u32) -> Self {
        Self { 0: x }
    }
}

impl std::ops::BitOr for Letters {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { 0: self.0 | rhs.0 }
    }
}
impl std::ops::BitOrAssign for Letters {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    /// The original string used to create this word.
    /// Multiple String if there are anagrams in the word list.
    string: Vec<String>,

    letters: Letters,
}

impl Word {
    pub fn add_word(&mut self, other: &Self) {
        self.string.extend_from_slice(&other.string)
    }
}

impl std::hash::Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.letters.hash(state);
    }
}

pub enum StringToWordError {
    BadLength,
    UnknownChar,
    DuplicateChar,
}

impl TryFrom<String> for Word {
    type Error = StringToWordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut out = Self {
            string: vec![value],
            letters: Letters::new(),
        };
        let mut len = 0;
        for char in out.string[0].chars() {
            match get_letter_index(char) {
                None => {
                    return Err(Self::Error::UnknownChar);
                }
                Some(index) => {
                    // the letter was recognized
                    let new_letter = Letters::from(1 << index);
                    if out.letters.shares_letters(new_letter) {
                        return Err(Self::Error::DuplicateChar);
                    };
                    out.letters = out.letters | new_letter;
                    len += 1;
                    if len > 5 {
                        return Err(Self::Error::BadLength);
                    }
                }
            };
        }
        if len != 5 {
            return Err(Self::Error::BadLength);
        }
        Ok(out)
    }
}

impl TryFrom<&String> for Word {
    type Error = StringToWordError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut out = Self {
            string: vec![value.clone()],
            letters: Letters::new(),
        };
        let mut len = 0;
        for char in out.string[0].chars() {
            match get_letter_index(char) {
                None => {
                    return Err(Self::Error::UnknownChar);
                }
                Some(index) => {
                    // the letter was recognized
                    let new_letter = Letters::from(1 << index);
                    if out.letters.shares_letters(new_letter) {
                        return Err(Self::Error::DuplicateChar);
                    };
                    out.letters = out.letters | new_letter;
                    len += 1;
                    if len > 5 {
                        return Err(Self::Error::BadLength);
                    }
                }
            };
        }
        if len != 5 {
            return Err(Self::Error::BadLength);
        }
        Ok(out)
    }
}

impl From<&Word> for Letters {
    fn from(word: &Word) -> Self {
        word.letters
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.letters == other.letters
    }
}

impl Eq for Word {}

impl std::fmt::Display for Word {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut text = String::new();
        for s in self.string.iter() {
            text += s;
            text += ", ";
        }
        let _ = write!(formatter, "{}", text);
        Ok(())
    }
}

fn get_letter_index(letter: char) -> Option<usize> {
    match letter {
        'A'..='Z' => Some(letter as usize - 65),
        'a'..='z' => Some(letter as usize - 97),
        _ => None,
    }
}
