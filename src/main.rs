use text_io::read;

fn main() {
    println!("Welcome to Blackjack");
    user_menu();
}

pub fn user_menu(){
    println!("Select an option");
    println!("1) Play");
    let user_menu_input: String = read!();
    if user_menu_input == "1"{
        game();
    }
}

pub fn game(){
    println!("Game Started");
    get_hand();
}

pub fn get_hand(){
    // Tuple of cards:
    let cards = (2,3,4,5,6,7,8,9,10, "A", "K", "Q", "J");
    // Get two random values from tuple, assign to variable
    let user_cards = (cards.0, cards.9);
    // if user_cards == cards.9 {
    //     println!("You got an ace!");
    //     println!("Use as 1 or 11?");
    //     let user_select_ace: i32 = read!();
    // }
    println!("Your cards are |{}||{}|", cards.0, cards.9)

}

