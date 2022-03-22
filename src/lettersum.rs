use std::collections::BTreeMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;

pub fn challange() {
    assert_eq!(lettersum("".to_string()), 0);
    assert_eq!(lettersum("a".to_string()), 1);
    assert_eq!(lettersum("z".to_string()), 26);
    assert_eq!(lettersum("cab".to_string()), 6);
    assert_eq!(lettersum("excellent".to_string()), 100);
    assert_eq!(lettersum("microspectrophotometries".to_string()), 317);
    //load enable1 wordlist
    let path = Path::new("enable1.txt");
    let display = path.display();
    
    //Load file into memory
    let file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };
    //Read file to String vector
    let buf = BufReader::new(file);
    let mut wordlist:Vec<String> = Vec::new();
    for line in buf.lines() {
        wordlist.push(line.unwrap());
    }
    //Make a timer
    let mut now:Instant;

    println!("Loaded enable1 wordlist with: {} words", wordlist.len());
    //Optional 1
    now = Instant::now();
    let wordwithsum = findwordwithsum(&wordlist, 319);
    println!("The word with size {} is {} used {}ms", lettersum(wordwithsum.to_string()), wordwithsum, now.elapsed().as_millis());
    //Optional 2
    now = Instant::now();
    let odds = oddsums(&wordlist);
    println!("There is {} numbers with odd sums used {}ms", odds, now.elapsed().as_millis());
    //Optional 3
    now = Instant::now();
    let common = mostcommonsum(&wordlist);
    println!("The biggest common sum is {} and it contains {} words used {}ms", common.0, common.1, now.elapsed().as_millis());
    //Optional 4
    now = Instant::now();
    let pairswithdiff = findpairwithdiff(&wordlist, 11);
    println!("Found a pair with same sum and diff off {}: {}:{} used {}ms", pairswithdiff.2, pairswithdiff.0, pairswithdiff.1, now.elapsed().as_millis());
    //Optional 5
    now = Instant::now();
    let nodupes = diffletters(&wordlist, 188);
    println!("Found a pair with no duplicate characters: {}:{} used {}ms", nodupes.0, nodupes.1, now.elapsed().as_millis());
    //Optinal 6
    now = Instant::now();
    let biggestlist = makebiggestlist(&wordlist);
    println!("Here is the biggest list i could find: {:?} used {}ms", biggestlist, now.elapsed().as_millis());

}

fn lettersum(letters: String) -> u32{
    let mut sum: u32 = 0;
    //Calculate offset so letter a is the 0th letter of the ascii table
    let offset: u32 = ('a' as u32) - 1;
    for c in letters.chars(){
        //Convert char to ascii value u32 overkill
        let val: u32 = c as u32;
        sum += val - offset;
    }
    return sum;
}

fn findwordwithsum(wordlist: &Vec<String>, size: u32) -> String {
    for word in wordlist {
        if lettersum(word.to_string()) == size{
            return word.to_string();
        }
    }
    return String::new();
}


fn oddsums(wordlist: &Vec<String>) -> u32 {
    //check numbers with uneven sum
    let mut odds: u32 = 0;
    for word in wordlist {
        if lettersum(word.to_string()) % 2 != 0{
            odds += 1;
        }
    }

    return odds;
}

fn mostcommonsum(wordlist: &Vec<String>) -> (u32, u32) {
    //Create hashmap to keep track of sums
    let mut allsums: HashMap<u32, u32> = HashMap::new();
    //Populate the hashmap
    for word in wordlist {
        let wordsum = lettersum(word.to_string());
        *allsums.entry(wordsum).or_insert(0) += 1;
    }
    //Rust magic to pop you the biggest key
    let biggestkey = allsums.iter().max_by_key(|key | key.1).unwrap();
    return (*biggestkey.0, *biggestkey.1);
}

fn findpairwithdiff(wordlist: &Vec<String>, diff: usize) -> (String, String, usize) {
    //Generate simelar hashmap but store words in a vector (GIGA MEMORY USAGE)
    let mut pairs: HashMap<u32, Vec<String>> = HashMap::new();
    for word in wordlist {
        let wordsum = lettersum(word.to_string());
        if pairs.contains_key(&wordsum){
            //bad error handeling D:
            pairs.get_mut(&wordsum).unwrap().push(word.to_string())
        } else {
            pairs.insert(wordsum, vec!(word.to_string()));
        }
    }

    //Nested loop probally a cleaner way to do it
    //We save the smallest len we find and biggest then compare
    for pair in pairs {
        let mut smallest = String::from("");
        let mut biggest = String::from("");
        for s in pair.1{
            //Need to init smallest with a real value
            if smallest.len() == 0{
                smallest = s.to_string();
            }
            if smallest.len() > s.len(){
                smallest = s.to_string();
            }
            if biggest.len() < s.len() {
                biggest = s.to_string();
            }
        }
        if biggest.len() - smallest.len() >= diff && biggest != "biodegradabilities" && smallest != "zyzzyva" {
            return (biggest.to_string(), smallest.to_string(), biggest.len() - smallest.len());
        }
    }
    //Default return if we find nothing
    return ("".to_string(),"".to_string(), 0);
}
//This one is actually pretty hard to do fast without using alot of memory
//just gonne do this with a double loop hashmap combo, very inefficent but can make it a bit better
//with breaking the look on a non match
fn diffletters(wordlist: &Vec<String>, min: u32) -> (String, String){
    //Same hashmap as diffpairs
    let mut pairs: HashMap<u32, Vec<String>> = HashMap::new();
    for word in wordlist {
        let wordsum = lettersum(word.to_string());
        if pairs.contains_key(&wordsum){
            //bad error handeling D:
            pairs.get_mut(&wordsum).unwrap().push(word.to_string())
        } else {
            pairs.insert(wordsum, vec!(word.to_string()));
        }
    }
    //Alot of loops, but good breakage so might not run like shit :)
    for pair in pairs {
        if pair.0 < min  {
            continue;
        }
        for s1 in &pair.1 {
            let mut dupechar = false;
            for s2 in &pair.1 {
                if s1 == s2 {
                    continue;
                }
                if dupechar == true {
                    dupechar = false;
                    continue;
                }
                for c in s1.chars(){
                    if s2.contains(c){
                        dupechar = true;
                        break;
                    }
                }
                if dupechar == false{
                    return (s1.to_string(), s2.to_string());
                }
            }
        }
    }
    return (String::new(), String::new());
}

fn makebiggestlist(wordlist: &Vec<String>) -> Vec<String> {
    let mut words:Vec<String> = Vec::new();
    //Same using a BTreeMap here, not as fast as hashmap but its sorted so we can do some smart ittering
    let mut pairs: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for word in wordlist {
        let wordsum = lettersum(word.to_string());
        if pairs.contains_key(&wordsum){
            //bad error handeling D:
            pairs.get_mut(&wordsum).unwrap().push(word.to_string())
        } else {
            pairs.insert(wordsum, vec!(word.to_string()));
        }
    }
    //Iterate in reverse order, biggest lettersum first
    //we just find a len in the sum then pick a len we dont have in the list
    //We could also add some logic here to prefer the bigger lens
    for pair in pairs.iter().rev() {
        let mut biggestlen = String::new();
        for s in pair.1 {
            let mut dupelen = false;
            for s2 in &words {
                if s2.len() == s.len(){
                    dupelen = true;
                    break;
                }
            }
            if dupelen {
                break;
            }

            if s.len() > biggestlen.len() {
                biggestlen = s.to_string();
            }
            words.push(biggestlen.to_string());
        }
    }

    return words;
}