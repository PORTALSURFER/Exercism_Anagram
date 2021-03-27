use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    // Get all letters in word

    let mut found_word_buffer: Vec<char> = Vec::new();
    let mut found_anagrams: Vec<&str> = Vec::new();
    let mut anagrams_hs: HashSet<&'a str> = HashSet::new();

    println!("\n\ntesting anagrams!");

    for &possible_anagram in possible_anagrams {
        // for every possible anagram
        for char in word.chars() {
            for pa_char in possible_anagram.chars() {
                // check if letter exists
                if char == pa_char {
                    // if yes, add letter to buffer array, and check if the next letter exists until all letters run out
                    found_word_buffer.push(pa_char);
                }
            }
        }
        // once letter run out, check if buffer array is the same size as the word.
        if found_word_buffer.len() == word.len() {
            // if it is, write the buffer array content to a list of found anagrams.
            found_anagrams.push(possible_anagram);
            anagrams_hs.insert(possible_anagram);
        }
    }

    // once all possible anagrams are tested, resurn the found anagrams as a hashset
    if found_anagrams.len() != 0 {
        println!("anagrams found {}", found_anagrams.len());

        for anagram in found_anagrams {
            println!("{}", anagram);
        }
    } else {
        println!("\n!!!No anagrams found!!!\n")
    }

    return anagrams_hs;
}
