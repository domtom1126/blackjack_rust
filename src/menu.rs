use text_io::read;
mod game;

pub fn user_menu(){
    println!("Select an option");
    println!("1) Play");
    let user_menu_input: String = read!();
    if user_menu_input == "1"{
        game::game();
    }
}