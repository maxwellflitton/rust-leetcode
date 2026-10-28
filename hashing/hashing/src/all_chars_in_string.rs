pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    // are all chars in the ransom_note in the magazine

    // create a buffer for string
    let mut counts = [0; 26];

    // increase the counts for the magazine
    for i in magazine.bytes() {
        let buffer_index = (i - b'a') as usize;
        counts[buffer_index] += 1;
    }

    // make sure there's always a char for the note
    for i in ransom_note.bytes() {
        let buffer_index = (i - b'a') as usize;
        counts[buffer_index] -= 1;

        // if the count goes below zero you know that there isn't enough of
        // those chars for the note
        if counts[buffer_index] < 0 {
            return false;
        }
    }

    true
}
