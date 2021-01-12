fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

// fn dna_to_rna(dna: &str) -> String {
//     dna.chars().map(char_conversion).collect()
// }

// fn char_conversion(c: char) -> char {
//     if c == 'T' {
//         return 'U';
//     }
    
//     c
// }