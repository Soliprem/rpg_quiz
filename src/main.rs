use std::{io, u8};

fn get_user_choice(question: [&str; 5]) -> u8 {
    let q = question[0];
    let opt1 = question[1];
    let opt2 = question[2];
    let opt3 = question[3];
    let opt4 = question[4];
    loop {
        let mut input = String::new();
        println!(
            "{}\n1. {}\n2. {}\n3. {}\n4. {}",
            q, &opt1, &opt2, &opt3, &opt4
        );
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
    let choices: [[&str; 5]; 12] = [ // Expanded slightly to 12 for better nuance
    [ // Q1: World Preference (Existing - Slightly tweaked D)
        "What kind of world do you find most interesting for an adventure?",
        "A dark, rain-slicked futuristic city wrestling with artificial life and moral decay.", // BR - More specific noir tech
        "A vast fantasy realm brimming with magic, diverse peoples, ancient ruins, and legendary monsters.", // PF - Emphasizes scope
        "A harsh, unforgiving landscape (wilds, space, etc.) where survival and personal promises are key.", // IS - Broadens beyond just landscape
        "A gritty, high-tech metropolis dominated by corporations, cybernetics, and street-level survival.", // CPR - Clearer focus
    ],
    [ // Q2: Core Theme (Existing - Good as is)
        "Which of these general themes appeals to you the most in a story?",
        "Investigating complex mysteries and making difficult choices with ambiguous outcomes.", // BR
        "Embarking on epic quests, battling powerful foes, and achieving heroic feats.", // PF
        "Forging your own path in a dangerous world, driven by personal commitments (vows).", // IS - Highlighting vows
        "Undertaking high-stakes missions, making a name for yourself, and fighting against the system.", // CPR
    ],
    [ // Q3: Preferred Activities (Existing - Slightly tweaked C & D)
        "When playing, what kind of activities sound most engaging?",
        "Gathering clues, interviewing characters, and piecing together complex situations.", // BR
        "Exploring dungeons, fighting monsters tactically, and discovering powerful artifacts.", // PF
        "Journeying through challenging environments, overcoming obstacles using wits, and fulfilling personal goals.", // IS
        "Executing daring jobs, engaging in fast-paced combat, and navigating social webs.", // CPR
    ],
    [ // Q4: Rules Feel (Revised Q4 - Focus on impact, not just complexity)
        "How do you prefer game rules to feel during play?",
        "Subtle and atmospheric, supporting the mood and story without demanding constant attention.", // BR (Year Zero)
        "Comprehensive and tactical, offering lots of options for strategy and character abilities.", // PF
        "Narrative-focused, directly triggering story moments and consequences based on dice rolls.", // IS (PbtA-style)
        "Action-oriented and intuitive, enabling dynamic scenes and impactful choices without excessive crunch.", // CPR (Interlock)
    ],
    [ // Q5: Character Appeal (Existing - Tweaked C)
        "What kind of character sounds most compelling to play?",
        "An introspective individual skilled at observation, deduction, and navigating moral grey areas.", // BR
        "A capable hero defined by their class, skills, and role within an adventuring party (even a small one).", // PF
        "A resilient survivor, defined more by their actions, experiences, and sworn oaths than by a class.", // IS
        "A stylish and skilled operator (merc, netrunner, etc.) carving out a niche in a dangerous world.", // CPR
    ],
    [ // Q6: Desired Atmosphere (Existing - Good as is)
        "What kind of overall atmosphere or feeling would you like our game sessions to have?",
        "Intriguing and suspenseful, with a sense of mystery and philosophical questions.", // BR
        "Exciting and adventurous, with a feeling of heroism and discovery in a magical world.", // PF
        "Immersive and personal, focused on the character's struggles, journeys, and the weight of their promises.", // IS
        "Intense and energetic, with high stakes, cool moments, and a rebellious attitude.", // CPR
    ],
    [ // Q7: Problem-Solving Approach (Existing - Good as is)
        "When faced with difficulties, what approach sounds most appealing?",
        "Using intellect, empathy, and investigation to find nuanced or non-violent solutions.", // BR
        "Employing teamwork, tactics, and special abilities to overcome challenges directly.", // PF
        "Relying on resourcefulness, determination, and tough choices to endure and succeed.", // IS
        "Utilizing street smarts, specialized skills (combat, tech, social), and decisive action.", // CPR
    ],
    [ // Q8: NPC Interaction Style (Revised Q8 - Sharpened focus)
        "What kind of interactions with non-player characters (NPCs) are most important to you?",
        "Deep, complex relationships and dialogues that reveal character and plot intricacies.", // BR
        "Meeting memorable allies, patrons, and villains who provide quests and challenges.", // PF
        "Encounters driven by the needs of survival, trust, and the harsh realities of the world.", // IS
        "Building a network of contacts, fixers, rivals, and clients essential for getting things done.", // CPR
    ],
    [ // Q9: Character Creation Emphasis (Revised Q9 - Focus on *what* is customized)
        "What aspect of creating a character excites you most?",
        "Defining their personality, background, internal conflicts, and place in the world.", // BR
        "Choosing from many classes, feats, spells, and gear to build a mechanically unique hero.", // PF
        "Starting with a simple concept and discovering the character through their experiences and vows.", // IS
        "Crafting a unique style, role (like Netrunner, Solo), and acquiring cool gear/cybernetics.", // CPR
    ],
    [ // Q10: Preferred Pace (Existing - Good as is)
        "What kind of pace do you generally prefer in games or stories?",
        "A more deliberate pace with time for investigation, reflection, and character interaction.", // BR
        "A varied pace mixing exploration, strategic combat, social encounters, and downtime.", // PF
        "A pace that flows organically with the journey and the challenges sworn upon.", // IS
        "A generally fast-paced experience driven by missions, action, and dealing with consequences.", // CPR
    ],
    [ // Q11: Tech vs Magic (New - Direct comparison)
        "Which element sounds more exciting to incorporate into your character and the world?",
        "Grounded, near-future tech like advanced forensics, replicants, spinners (flying cars).", // BR
        "High fantasy magic: spellcasting, enchanted items, divine powers, mythical beasts.", // PF
        "A gritty world with little or no magic, or perhaps only subtle, mysterious rituals.", // IS
        "Transhuman tech: cybernetic limbs, neural interfaces, high-tech weapons, netrunning.", // CPR
    ],
    [ // Q12: Duet Focus (New - Tailored for 1-on-1)
        "In a game with just you and the GM, what dynamic sounds best?",
        "A focused narrative on my character's cases, moral dilemmas, and personal stakes.", // BR
        "My character as the primary hero, perhaps with key NPC allies joining the adventures.", // PF (adapting party)
        "A deeply personal story centered on my character's solitary journey and sworn quests.", // IS (strong duet fit)
        "My character navigating dangerous gigs and relationships within a vibrant, often hostile, city.", // CPR
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
        println!("You chose option {}!\n", choice);
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
