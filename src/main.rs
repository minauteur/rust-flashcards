            use std::sync::{Arc, Mutex};
use std::ops::DerefMut;
use std::borrow::ToOwned;

#[derive(Clone, Debug)]
struct GlobalDecks {
    all_decks: Arc<Mutex<Vec<Deck>>>,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct FlashCard {
    id: i64,
    term: String,
    def: String,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct Deck {
    id: i64,
    uploader: UserInfo,
    name: String,
    tags: Vec<String>,
    contents: Vec<FlashCard>,
}
#[derive(Eq, PartialEq, Clone, Debug)]
struct UserInfo {
    email: String,
}

fn main() {
    let mut global_decks: GlobalDecks = GlobalDecks {
        all_decks: Arc::new(Mutex::new(Vec::new())),
    };
    let mut add_to_global = match global_decks.all_decks.lock() {
        Ok(vec) => vec,
        Err(e) => {
            return println!("Error: {:?}", e);
        }
    };
    let address_1: String = "thom@gmail.com".to_string();
    let user_1: UserInfo = UserInfo { email: address_1 };
    let address_2: String = "thom2@gmail.com".to_string();
    let user_2: UserInfo = UserInfo { email: address_2 };


    let deck_name_1 = "My first deck".to_string();
    let deck_name_2 = "My second deck".to_string();

    let t_1 = vec!["questions".to_string(), "generic".to_string()];
    let t_2 = vec!["questions".to_string(), "specific".to_string()];

    let q_1 = "A is the term".to_string();
    let a_1 = "B is the definition".to_string();

    let q_2 = "C is the term".to_string();
    let a_2 = "D is the definition".to_string();

    let q_3 = "E is the term".to_string();
    let a_3 = "F is the definition".to_string();

    let q_4 = "G is the term".to_string();
    let a_4 = "H is the definition".to_string();

    let card_1: FlashCard = FlashCard {
        id: 01,
        term: q_1,
        def: a_1,
    };

    let card_2: FlashCard = FlashCard {
        id: 02,
        term: q_2,
        def: a_2,
    };
    let card_3: FlashCard = FlashCard {
        id: 03,
        term: q_3,
        def: a_3,
    };

    let card_4: FlashCard = FlashCard {
        id: 04,
        term: q_4,
        def: a_4,
    };
    let mut deck_contents_1: Vec<FlashCard> = vec![card_1, card_2];
    let mut deck_contents_2: Vec<FlashCard> = vec![card_3, card_4];
    let mut deck_item_1: Deck = Deck {
        id: 01,
        uploader: user_1,
        name: deck_name_1,
        tags: t_1,
        contents: deck_contents_1,
    };
    let mut deck_item_2: Deck = Deck {
        id: 02,
        uploader: user_2,
        name: deck_name_2,
        tags: t_2,
        contents: deck_contents_2,
    };
    add_to_global.deref_mut().push(deck_item_1);
    add_to_global.deref_mut().push(deck_item_2);
    let mut n = 0;
    let mut deref_vec = add_to_global.clone().to_owned();

    while n < add_to_global.len() {
        let mut everything = &deref_vec[n];
        println!("Deck Name: {:?}", &everything.name);
        println!(
            "Number of cards in Deck: {:?}",
            &everything.contents.len().to_string()
        );
        let mut x = 0;
        while x < everything.contents.len() {
            let mut next_card = &everything.contents[x];
            println!(
                "Card Id: {:?} \n Term: {:?}, Def: {:?}",
                &next_card.id,
                &next_card.term,
                &next_card.def
            );
            x = x + 1;
        }
        n = n + 1;
    }


    //for active_card in &deck_vec.contents {
    // let mut i = 0;
    // while i< deck_vec.contents.len() {
    //     let mut next_card = &deck_vec.contents[i];
    //     println!("Q: {:?}, A: {:?}", &next_card.term, &next_card.def);
    //     i = i+1;
    // }
    //}
    // println!("Q: {:?}, A: {:?}", &card_1.term, &card_1.def);
}
