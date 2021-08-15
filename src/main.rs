use std::io;
use std::collections::HashMap;

struct Player
{
    name: String,
    health: u32,
    have_cup: bool
}

fn main() {
    // position of where we are in our game
    let mut _pos = 0;

    let mut room0 = HashMap::new();
    room0.insert(String::from("look"), String::from("There's a door, and a cup in the room."));
    room0.insert(String::from("take"), String::from("Take what?"));
    room0.insert(String::from("take_cup"), String::from("You take the cup."));
    room0.insert(String::from("leave_room"), String::from("You exit the room."));
    room0.insert(String::from("go_door"), String::from("You exit the room."));

    let mut room1 = HashMap::new();
    room1.insert(String::from("look"), String::from("There's a door back to the room behind you, ahead of you is the kitchen."));
    room1.insert(String::from("take"), String::from("There's nothing for you to take!"));
    room1.insert(String::from("go_back"), String::from("You go back to your room."));
    room1.insert(String::from("go_kitchen"), String::from("You head to the kitchen."));
    
    let mut room2 = HashMap::new();
    room2.insert(String::from("look"), String::from("There's a fridge ahead of you, and the hallway behind you."));
    room2.insert(String::from("take"), String::from("There's nothing for you to take!"));
    room2.insert(String::from("go_back"), String::from("You go back to the hallway."));
    room2.insert(String::from("go_fridge"), String::from("You cannot enter the fridge. Try something else."));
    room2.insert(String::from("open_fridge"), String::from("You open the fridge."));
    room2.insert(String::from("look_fridge"), String::from("In the fridge is a bottle of coke."));
    room2.insert(String::from("pour_drink"), String::from("You have poured yourself a nice refreshing drink! Congrats! Thanks for playing."));

    let v = vec![room0, room1, room2];

    println!("Hello adventurer! Please enter your name!");

    let mut name = String::new();
    name = take_commands(name);

    // if
    let mut player = create_player(name);

    println!("Hello, {}! Your health is {}", &player.name, &player.health);

    loop
    {
        println!("What would you like to do?");
        let mut cmd = String::new();
        cmd = take_commands(cmd);
        let key = cmd.replace(" ", "_");
        // is_running = if cmd == "exit" { false } else { true };
        if cmd == "exit"
        {
            break;
        }
        if v[_pos].contains_key(&key)
        {
            if key == "pour_drink" && player.have_cup
            {
                println!("{}", v[_pos][&key]);
                break;
            }
            if key == "pour_drink" && !player.have_cup
            {
                println!("You pour your drink all over the floor. Next time, take a cup with you.");
                break;
            }
            println!("{}", v[_pos][&key]);
            if key == "take_cup" { player.have_cup = true }
            if key == "go_back" && _pos > 0 { _pos -= 1; }
            if key == "go_kitchen" || key == "leave_room" || key == "go_door" { _pos += 1; }
        }
        else { println!("I don't understand {}", cmd); }
    }
    println!("Exiting...");
}

fn take_commands(cmd: String) -> String
{
    let mut cmd = cmd;
    io::stdin()
        .read_line(&mut cmd)
        .expect("Could not read command");
    return cmd.trim_end().to_string();
}

fn create_player(p_name: String) -> Player
{
    let player = Player
    {
        name: String::from(p_name),
        health: 100,
        have_cup: false
    };

    return player;
}