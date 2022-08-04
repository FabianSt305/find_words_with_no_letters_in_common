# find_words_with_no_letters_in_common

Inspired by Matt Parker's video (https://youtu.be/_-AfhLQfb6w), this is a rather simple program written in rust that reads a word list from a txt file and then iterates over it, trying to find five words of five letters with no letters in common as fast as possible.


# Differences between original and fork
- Store words as a single u32 instead of a [bool; 26] for huge speedups
- Reduced cloning
- Deduplication uses HashMap instead of Vec for lookup
- Added Argument parsing
- Added progress bar to escape printing hell
- Added parallelism

# What you should know

Please adjust the -i flag according to where your word list is. If the file cannot be found, access to it is denied or there is any other io error, the programm will tell you to download the word list from a url (which is copied from matt parker's video description).

Don't actually do this. The file is in this repository under `words_alpha.txt`


# How

The first step is to abstract all the words into the Word struct. It stores 26 bits, one for each letter (only a-z supported) in a u32. By default, all of these will be false. By initializing a Word using Word::try_from(string), exactly 5 of them will be set to true. These are the letters that the word contains. If the word does not contain exactly 5 different letters, Word::try_from(string) errors, meaning that the Word was not valid. This creates a list of Words which we can iterate over.


To ensure that all words will be checked and none will be missed, there are 5 nested for loops. Because the inner for loops use 'i in _..i' to iterate, there will never be a situation where two 'i's will be the same (i.e. we are comparing one word to itself) nor will there be a situation where a comparison will take part twice (one time comparing a to b, then later b to a).

If two words are found which have any overlap in letters, we immediately stop searching and move on to the next word pair.

My solution to making this rather efficient is the following: (my code here is not super expressive with til1-til4, the following explanation will completely disregard why there are multiple of these)

Instead of comparing words to one another, we create a new 'word'.
This word breaks the rule normal words have, in that it is never required to have only five letters.

Given any number of words n with 5 letters each, this special word will consist of all of the letters.


# Result

Running this algorithm with Matt's word list (`words_alpha.txt`, 359930 words, 5977 unique) on my laptop takes less than a minute.

And yes, it does actually work. This is the output:

# Output

```
cargo run --release -- -i words_alpha.txt
```

```
vejoz, 
hdqrs, 
flang, 
twick, 
bumpy, 

fldxt, 
zingy, 
jumba, 
wreck, 
qophs, 

fldxt, 
wrack, 
benzo, bonze, 
jimpy, 
vughs, 
```

Notice that benzo and bonze both consist of the same letters, which is why they were combined into one "word".
We can also see FLDXT and HDQRS which played a prominent role in the video.
