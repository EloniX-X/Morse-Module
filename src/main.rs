
mod morse {
    use std::{collections::HashMap};
    fn translate (option: bool, text: &str) -> String {
        let cs: Vec<&str> = text.split(" ").collect();
        let texts: Vec<&str> = cs.iter().map(|&x| if x == "" { "/" } else { x }).collect();
        let translate = HashMap::from([
            ('a', ".-"), 
            ('b', "-..."), 
            ('c', "-.-."), 
            ('d', "-.."), 
            ('e', "."), 
            ('f', "..-."), 
            ('g', "--."), 
            ('h', "...."), 
            ('i', ".."), 
            ('j', ".---"), 
            ('k', "-.-"), 
            ('l', ".-.."), 
            ('m', "--"), 
            ('n', "-."), 
            ('o', "---"), 
            ('p', ".--."), 
            ('q', "--.-"), 
            ('r', ".-."), 
            ('s', "..."), 
            ('t', "-"), 
            ('u', "..-"), 
            ('v', "...-"), 
            ('w', ".--"), 
            ('x', "-..-"), 
            ('y', "-.--"), 
            ('z', "--.."), 
            ('0', "-----"), 
            ('1', ".----"), 
            ('2', "..---"), 
            ('3', "...--"), 
            ('4', "....-"), 
            ('5', "....."), 
            ('6', "-...."), 
            ('7', "--..."), 
            ('8', "---.."), 
            ('9', "----."),
            (' ', "/"),
        ]);
        if option {
            let mut text = "".to_owned();
            for ent in texts {
                let res: String = ent.chars().map(|f| translate.get(&f).unwrap().to_string()).collect::<Vec<String>>().join(" ");
                text.push_str(&(res + " / "));
            }
            text
        } else {
            let mrs: HashMap<&str, char> = translate.iter()
                .map(|(&letter, &code)| (code, letter)) 
                .collect();

            let res: String = texts.iter()
            .map(|f| mrs.get(f).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("");
        
        res
    }

    }
    pub fn decrypt(decrypt: &str) -> String {
        let h = translate(false, decrypt);
        h
    }

    pub fn encrypt(encrypt: &str) -> String {
        let h = translate(true, encrypt);
        h
    }
}




fn main() {
    let h = morse::encrypt("eloni hey ok");
    println!("{}", h);
    let b = morse::decrypt(&h);
    println!("{}", b);
}
