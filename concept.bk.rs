//I began this project in the play.integer32 rust playground as a proof of concept. That code is as follows:
#[derive(Eq, PartialEq, Clone, Debug)]
struct FlashCard {
id: i64,
term: String,
def: String,
}

struct Deck {
id: i64,
uploader: UserInfo,
name: String,
tags: Vec<String>,
contents: Vec<FlashCard>
}

struct UserInfo {
email: String
}

fn main() {
let address: String = "thom@gmail.com".to_string();
let user: UserInfo = UserInfo {
email: address
};

let deck_name = "My first deck".to_string();
let t = vec!("questions".to_string(), "generic".to_string());


let q_1 = "A is the term".to_string();
let a_1 = "B is the definition".to_string();

let q_2 = "C is the term".to_string();
let a_2 = "D is the definition".to_string();

let card_1: FlashCard = FlashCard {
    id: 01,
    term: q_1,
    def: a_1
};

let card_2: FlashCard = FlashCard {
    id: 02,
    term: q_2,
    def: a_2
};
let mut cards: Vec<FlashCard> = vec!(card_1, card_2);

let mut deck_vec: Deck = Deck {
    id: 01,
    uploader: user,
    name: deck_name,
    tags: t,
    contents: cards
};


//for active_card in &deck_vec.contents {
    let mut i = 0;
    while i< deck_vec.contents.len() {
        let mut next_card = &deck_vec.contents[i];
        println!("Q: {:?}, A: {:?}", &next_card.term, &next_card.def);
        i = i+1;
    }
//}
// println!("Q: {:?}, A: {:?}", &card_1.term, &card_1.def);

}
