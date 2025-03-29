use std::{io, u8};

fn get_user_choice(question: [&str; 4]) -> u8 {
    let q = question[0];
    let opt1 = question[1];
    let opt2 = question[2];
    let opt3 = question[3];
    loop {
        let mut input = String::new();
        println!("{}\n1. {}\n2. {}\n3. {}", q, &opt1, &opt2, &opt3);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();

        match trimmed.parse::<u8>() {
            Ok(num) if (1..=3).contains(&num) => return num,
            _ => println!("Invalid choice. Please enter 1, 2, or 3"),
        }
    }
}

fn main() {
    println!("Time to figure out which RPG you prefer!");
    let mut bladerunner: u8 = 0;
    let mut pathfinder: u8 = 0;
    let mut ironsworn: u8 = 0;
    let choices: [_; 10] = [
        [
            "What genre do you prefer?",
            "Noir (Moral dilemmas, mystery, and crime.)", // Blade Runner
            "Heroic (Grand adventures, good vs evil.)",   // Pathfinder
            "Personal Narrative (Character-driven storytelling.)", // Ironsworn
        ],
        [
            "Which setting do you prefer?",
            "Dark Sci-Fi (Cyberpunk, corporations, AI, neon-lit cities.)", // Blade Runner
            "High Fantasy (Magic, kingdoms, elves, and dragons.)",         // Pathfinder
            "Dark Fantasy (Gritty, struggle, survival, fate-driven.)",     // Ironsworn
        ],
        [
            "How do you feel about dice mechanics?",
            "Simple rolls, no surprises—just roleplay.", // Blade Runner (D6-based)
            "I enjoy rolling lots of dice and crunchy mechanics.", // Pathfinder (D20)
            "I like rolling with consequences and uncertain outcomes.", // Ironsworn (D10 + moves)
        ],
        [
            "What kind of character arcs interest you the most?",
            "Struggles with morality, identity, and power in a dystopian world.", // Blade Runner
            "Epic heroes who grow stronger and change the world.",                // Pathfinder
            "Personal journeys shaped by choices, fate, and hardship.",           // Ironsworn
        ],
        [
            "How structured do you want the game to be?",
            "Tightly focused on solving cases with a clear framework.", // Blade Runner
            "A sandbox world full of dungeons, quests, and combat.",    // Pathfinder
            "A loose framework where story and improvisation guide play.", // Ironsworn
        ],
        [
            "How important is combat to you?",
            "Combat should be quick and minimal, with focus on investigation.", // Blade Runner
            "I love tactical combat, building strong characters, and planning battles.", // Pathfinder
            "Combat is part of the story, but survival and stakes are more important than balance.", // Ironsworn
        ],
        [
            "How do you feel about rule complexity?",
            "I prefer a streamlined system that lets me focus on story and mystery.", // Blade Runner
            "I love deep mechanics, character builds, and optimizing my character.",  // Pathfinder
            "I prefer flexible rules that let the story take the lead.",              // Ironsworn
        ],
        [
            "How do you feel about a GM (Game Master) guiding the story?",
            "A GM-driven mystery with clear objectives sounds great!", // Blade Runner
            "I like a traditional GM-led adventure with lots of prep and structure.", // Pathfinder
            "I'm fine with either a GM or co-op storytelling where players drive the world.", // Ironsworn
        ],
        [
            "What kind of challenges do you enjoy most?",
            "Solving mysteries, navigating moral dilemmas, and making tough choices.", // Blade Runner
            "Overcoming enemies, puzzles, and exploring new locations.",               // Pathfinder
            "Surviving, building my character’s legend, and improvising my fate.",     // Ironsworn
        ],
        [
            "How do you feel about character failure?",
            "Failure should be part of the mystery—bad choices have consequences.", // Blade Runner
            "Failure should be fair, but I want opportunities to recover and keep going.", // Pathfinder
            "Failure drives the story forward and shapes my character’s journey.", // Ironsworn
        ],
    ];
    for question in choices {
        let choice = get_user_choice(question);
        match choice {
            1 => bladerunner += 1,
            2 => pathfinder += 1,
            3 => ironsworn += 1,
            _ => unreachable!(),
        }
        println!("You chose option {}!", choice);
    }

    let (max_rpg, max_score) = if bladerunner >= pathfinder && bladerunner >= ironsworn {
        ("Blade Runner RPG", bladerunner)
    } else if pathfinder >= ironsworn {
        ("Pathfinder", pathfinder)
    } else {
        ("Ironsworn", ironsworn)
    };

    println!(
        "Based on your choices, {} is the best fit with a score of {}!",
        max_rpg, max_score
    );
}
