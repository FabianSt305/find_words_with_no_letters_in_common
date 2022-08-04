pub struct Word {
    /// The original string used to create this word.
    string: Vec<String>,
    /// If this is true at a certain index, it means that that letter was used in the word. By definition, if this Word was creates using from_string, exactly five elements will be true, while all others are false.
    letters: u32,
}

const BIT_MASK_26: u32 = 0x03ffffff; // 0000 0011 1111 1111 1111 1111 1111 1111

impl Word {
    pub fn from_letters(l: u32) -> Self {
        Self {
            string: Vec::new(),
            letters: l,
        }
    }
    pub fn from_string(str: String) -> Option<Self> {
        let mut out = Self {
            string: vec![str],
            letters: 0,
        };
        let mut len = 0;
        for char in out.string[0].chars() { // iterate over the word's utf-8 chars
            match get_letter_index(char) { // see if the character is recognized, and if so, which letter it is
                Some(index) => { // the letter was recognized
                    let bit_mask = 1<<index;
                    if out.letters & bit_mask != 0 {
                        return None; // because a letter was used twice (using this word makes no sense)
                    };
                    out.letters |= bit_mask;
                    len += 1;
                    if len > 5 {
                        return None; // because the word is too long
                    }
                },
                None => { // unknown letter
                    return None; // because the word contains unsupported characters
                },
            };
        };
        if len < 5 { return None; }; // because the word is too short
        Some(out) // word is valid, return it
    }

    pub fn add_str(&mut self, str: String) {
        self.string.push(str);
    }
    /*
    pub fn used_letters(w1: &Self, w2: &Self, w3: &Self, w4: &Self, w5: &Self) -> u8 {
        (w1.letters | w2.letters | w3.letters | w4.letters | w5.letters).count_ones() as u8
    }
    pub fn has_no_duplicates(w1: &Self, w2: &Self, w3: &Self, w4: &Self, w5: &Self) -> bool {
        Self::used_letters(w1, w2, w3, w4, w5) == 25 // if 25 letters were used, since every word uses exactly 5 letters and duplicate letters will only add 1 to the count, there were no duplicates.
    }
    */
    pub fn clone_letters(&self) -> Self {
        Self { 
            string: Vec::with_capacity(0),
            letters: self.letters
        }
    }
    pub fn num_shared_letters(&self, other: &Self) -> u8 {
        (self.letters & other.letters).count_ones() as u8
    }
    pub fn shares_letters(&self, other: &Self) -> bool {
        (self.letters & other.letters) != 0
    }
    pub fn combine_letters(&self, other: &Self) -> Self {
        Self {
            string: Vec::with_capacity(0),
            letters: self.letters | other.letters,
        }
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
        write!(formatter, "{}", text);
        Ok(())
    }
}

pub fn get_letter_index(letter: char) -> Option<usize> {
    match letter {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        'i' => Some(8),
        'j' => Some(9),
        'k' => Some(10),
        'l' => Some(11),
        'm' => Some(12),
        'n' => Some(13),
        'o' => Some(14),
        'p' => Some(15),
        'q' => Some(16),
        'r' => Some(17),
        's' => Some(18),
        't' => Some(19),
        'u' => Some(20),
        'v' => Some(21),
        'w' => Some(22),
        'x' => Some(23),
        'y' => Some(24),
        'z' => Some(25),
        _ => None,
    }
}

pub fn get_letter(index: usize) -> char {
    letters[index]
}

const letters: [char;26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];