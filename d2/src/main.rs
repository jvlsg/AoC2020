//Each line gives the password policy and then the password.
//The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
//How many passwords are valid?
use std::fs;
use std::str;
use regex::Regex;
use std::io;

struct PasswdPolicy {
    repeat_max: usize,
    repeat_min: usize,
    repeat_char: char,
    password: String,
}

impl PasswdPolicy {
    
    //PT1
    fn is_sled_valid(&self) -> bool{
        let mut count = 0;
        for c in self.password.chars(){
            if c==self.repeat_char {count+=1};
        }
        if count >= self.repeat_min && count <= self.repeat_max{
            return true;
        }
        false
    }

    //PT2
    //the Official Toboggan Corporate Authentication System works differently
    fn is_toboggan_valid(&self) -> bool{
        let chars: Vec<char> = self.password.chars().collect();
        //XOR - The character must be present in exactly one position
        (chars[self.repeat_min-1]==self.repeat_char) ^ (chars[self.repeat_max-1]==self.repeat_char)
    }
}

impl str::FromStr for PasswdPolicy {
    //Being quite lazy in terms of error handling. 
    //Just unwraping and hoping for the best
    type Err = io::Error;//Laziness to define a proper error type
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<repeat>[a-z]): (?P<password>[a-z]+)$").unwrap();
        let caps = re.captures(s).unwrap();
        
        let repeat_max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
        let repeat_min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
        let repeat_char: char = caps.name("repeat").unwrap().as_str().chars().next().unwrap();
        let password = caps.name("password").unwrap().as_str().to_string();
        
        Ok(PasswdPolicy{
            repeat_min,
            repeat_max,
            repeat_char,
            password,
        })
    }
}

fn valid_passwords_count(filename: &str, is_sled: bool) -> usize{
    if is_sled {
        fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| x.parse::<PasswdPolicy>().unwrap())
        .filter(|x| x.is_sled_valid() == true)
        .count()
    }
    else{
        fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| x.parse::<PasswdPolicy>().unwrap())
        .filter(|x| x.is_toboggan_valid() == true)
        .count()
    }
}

fn main() {
    println!("{} passwords are valid - Sled Policy",valid_passwords_count("input/input.txt",true));
    println!("{} passwords are valid - Toboggan Policy",valid_passwords_count("input/input.txt",false));
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn find_two_sled(){
        assert_eq!(valid_passwords_count("input/test.txt",true),2);
    }
    
    #[test]
    fn find_one_toboggan(){
        assert_eq!(valid_passwords_count("input/test.txt",false),1);
    }
}

