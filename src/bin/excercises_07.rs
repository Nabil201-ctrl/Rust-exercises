//Count vowels and consonants in a sentence.

fn main (){

}

fn count (input: String){
    let mut vowel_counter: u32;
    let mut consonants_counter: u32;
    
    for char in input.chars() {
        if char == 'a' | 'e' | 'i' | 'o' | 'u' {
            vowel_counter +=1;
        }
        else {
            consonants_counter +=1;
        }
    }

    
}