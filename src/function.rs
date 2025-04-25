pub fn update_str(string:&mut String){
    string.push_str(" world");
}

pub fn get_frist_word(sentence:String) -> String{
    let mut n_str = String::new();

    for char in sentence.chars(){
        if char == ' '{
            break;
        }
        n_str.push(char);
    }
    return n_str;
}

pub fn get_frist_char(sentence:&String) -> String{
    let char1 = sentence.chars().nth(0);

    match char1{
        Some(c)=> return c.to_string(),
        None => return "No frist word found".to_string(),
    }
}
