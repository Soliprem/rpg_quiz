use std::{io, u8};

fn get_user_choice(question: [&str; 5]) -> u8 {
    let q = question[0];
    let opt1 = question[1];
    let opt2 = question[2];
    let opt3 = question[3];
    let opt4 = question[4];
    loop {
        let mut input = String::new();
        println!("{}\n1. {}\n2. {}\n3. {}\n4. {}", q, &opt1, &opt2, &opt3, &opt4);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();

        match trimmed.parse::<u8>() {
            Ok(num) if (1..=4).contains(&num) => return num,
            _ => println!("Invalid choice. Please enter 1, 2, 3, or 4"),
        }
    }
}

fn main() {
    println!("Time to figure out which RPG you prefer!");
    let mut bladerunner: u8 = 0;
    let mut pathfinder: u8 = 0;
    let mut ironsworn: u8 = 0;
    let mut cyberpunk: u8 = 0;
    let choices: [[&str; 5]; 8] = [
    [
        "What kind of world do you find most interesting for an adventure?",
        "A dark, futuristic city with cool technology and tough choices.", // Bladerunner (Simplified)
        "A magical land with heroes, monsters, and grand quests.", // Pathfinder (Simplified)
        "A wild and lonely place where you have to rely on yourself and your promises.", // Ironsworn (Simplified)
        "A dangerous, exciting city in the near future with cybernetic enhancements and powerful corporations.", // CP:RED (Simplified)
    ],
    [
        "Which of these types of stories sounds most exciting to you?",
        "Solving mysteries and making difficult decisions with big consequences.", // Bladerunner (Simplified)
        "Being a hero, fighting evil, and discovering amazing things.", // Pathfinder (Simplified)
        "Exploring a harsh world, making your own way, and keeping your word.", // Ironsworn (Simplified)
        "Going on action-packed missions, looking cool, and standing up to the bad guys.", // CP:RED (Simplified)
    ],
    [
        "When you play a game, what kind of things do you enjoy doing the most?",
        "Figuring out clues and talking to people to solve problems.", // Bladerunner (Simplified)
        "Battling monsters and using special abilities.", // Pathfinder (Simplified)
        "Exploring new places and trying to survive in a challenging environment.", // Ironsworn (Simplified)
        "Having exciting fights and making a name for yourself.", // CP:RED (Simplified)
    ],
    [
        "How do you feel about rules in games?",
        "I prefer games where the story is the main focus, and the rules are simple.", // Bladerunner & Ironsworn (Combined for simplicity)
        "I like having lots of options for my character and clear rules for what I can do.", // Pathfinder & CP:RED (Combined for broader appeal)
        "I trust you to guide us, and I'm happy to learn as we go.", // General option for a new player
        "As long as the rules make sense, I'm okay with them.", // Another general option
    ],
    [
        "What kind of character would you enjoy playing as?",
        "Someone clever and good at solving problems.", // Bladerunner (Simplified)
        "A brave hero with special skills or powers.", // Pathfinder (Simplified)
        "Someone tough and resourceful who can handle anything.", // Ironsworn (Simplified)
        "Someone cool and skilled who can get things done.", // CP:RED (Simplified)
    ],
    [
        "What kind of feeling do you want our game sessions to have?",
        "Intriguing and like we're uncovering a secret.", // Bladerunner (Simplified)
        "Exciting and like we're on a big adventure.", // Pathfinder (Simplified)
        "Atmospheric and like we're facing challenges in a serious world.", // Ironsworn (Simplified)
        "Action-packed and like we're in the middle of a thrilling story.", // CP:RED (Simplified)
    ],
    [
        "How do you feel about you (as the GM) leading the story?",
        "I'd like you to guide us through a mystery or case.", // Bladerunner (Simplified)
        "I'd like you to take us on an adventure with challenges to overcome.", // Pathfinder (Simplified)
        "I'm happy to see where the story goes based on our choices.", // Ironsworn (Simplified - more player-driven)
        "I'm excited for you to set up cool missions and see what we do.", // CP:RED (Simplified)
    ],
    [
        "When things get tough in a game, what do you prefer?",
        "Trying to outsmart the problem or find a clever solution.", // Bladerunner (Simplified)
        "Facing the challenge head-on with strength and skill.", // Pathfinder (Simplified)
        "Focusing on surviving and making the best of a bad situation.", // Ironsworn (Simplified)
        "Getting into action and fighting our way through it.", // CP:RED (Simplified)
    ],
];
    for question in choices {
        let choice = get_user_choice(question);
        match choice {
            1 => bladerunner += 1,
            2 => pathfinder += 1,
            3 => ironsworn += 1,
            4 => cyberpunk += 1,
            _ => unreachable!(),
        }
        println!("You chose option {}!", choice);
    }

    let (max_rpg, max_score) =
        if bladerunner >= pathfinder && bladerunner >= ironsworn && bladerunner >= cyberpunk {
            ("Blade Runner RPG", bladerunner)
        } else if pathfinder >= ironsworn && pathfinder >= cyberpunk {
            ("Pathfinder", pathfinder)
        } else if cyberpunk >= ironsworn {
            ("Cyberpunk RED", cyberpunk)
        } else {
            ("Ironsworn", ironsworn)
        };

    println!(
        "Based on your choices, {} is the best fit with a score of {}!",
        max_rpg, max_score
    );
}
