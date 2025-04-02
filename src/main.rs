use std::{io, u8};

fn get_user_choice(question: [&str; 6]) -> u8 {
    let q = question[0];
    let opt1 = question[1];
    let opt2 = question[2];
    let opt3 = question[3];
    let opt4 = question[4];
    let opt5 = question[5];
    loop {
        let mut input = String::new();
        println!(
            "{}\n1. {}\n2. {}\n3. {}\n4. {}\n5. {}",
            q, &opt1, &opt2, &opt3, &opt4, &opt5
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();

        match trimmed.parse::<u8>() {
            Ok(num) if (1..=5).contains(&num) => return num,
            _ => println!("Invalid choice. Please enter 1, 2, 3, 4 or 5"),
        }
    }
}

fn main() {
    println!("Time to figure out which RPG you prefer!");
    let mut bladerunner: u8 = 0;
    let mut pathfinder: u8 = 0;
    let mut ironsworn: u8 = 0;
    let mut cyberpunk: u8 = 0;
    let mut gumshoe: u8 = 0;
    // Note: Array structure changed to [[&str; 6]; 12] to accommodate 5 systems.
    // Options generally map as: A=Bladerunner, B=Pathfinder2, C=Ironsworn, D=Cyberpunk:RED, E=GUMSHOE
    let choices: [[&str; 6]; 12] = [
        [
            // Q1: World Preference
            "What kind of world do you find most interesting for an adventure?",
            "A dark, rain-slicked futuristic city wrestling with artificial life and moral decay.", // A: Bladerunner
            "A vast fantasy realm brimming with magic, diverse peoples, ancient ruins, and legendary monsters.", // B: Pathfinder 2
            "A harsh, unforgiving landscape (wilds, space, etc.) where survival and personal promises are key.", // C: Ironsworn
            "A gritty, high-tech metropolis dominated by corporations, cybernetics, and street-level survival.", // D: Cyberpunk: RED
            "A world dense with secrets, where investigation uncovers conspiracies hidden beneath the surface of society, technology, or power structures.", // E: GUMSHOE
        ],
        [
            // Q2: Core Theme
            "Which of these general themes appeals to you the most in a story?",
            "Investigating complex mysteries and making difficult choices with ambiguous outcomes.", // A: Bladerunner
            "Embarking on epic quests, battling powerful foes, and achieving heroic feats.", // B: Pathfinder 2
            "Forging your own path in a dangerous world, driven by personal commitments (vows).", // C: Ironsworn
            "Undertaking high-stakes missions, making a name for yourself, and fighting against the system.", // D: Cyberpunk: RED
            "Unraveling complex mysteries step-by-step, where knowledge is power and finding the truth is the main goal.", // E: GUMSHOE
        ],
        [
            // Q3: Preferred Activities
            "When playing, what kind of activities sound most engaging?",
            "Gathering clues, interviewing characters, and piecing together complex situations.", // A: Bladerunner
            "Exploring dungeons, fighting monsters tactically, and discovering powerful artifacts.", // B: Pathfinder 2
            "Journeying through challenging environments, overcoming obstacles using wits, and fulfilling personal goals.", // C: Ironsworn
            "Executing daring jobs, engaging in fast-paced combat, and navigating social webs.", // D: Cyberpunk: RED
            "Methodically gathering clues using specific expertise, knowing you won't miss the vital leads, and using resources to dig deeper.", // E: GUMSHOE
        ],
        [
            // Q4: Rules Feel
            "How do you prefer game rules to feel during play?",
            "Subtle and atmospheric, supporting the mood and story without demanding constant attention.", // A: Bladerunner (Year Zero)
            "Comprehensive and tactical, offering lots of options for strategy and character abilities.", // B: Pathfinder 2
            "Narrative-focused, directly triggering story moments and consequences based on dice rolls.", // C: Ironsworn (PbtA-style)
            "Action-oriented and intuitive, enabling dynamic scenes and impactful choices without excessive crunch.", // D: Cyberpunk: RED (Interlock)
            "Streamlined for investigation, ensuring the story progresses, while other challenges might use resource management or simpler tests.", // E: GUMSHOE
        ],
        [
            // Q5: Character Appeal
            "What kind of character sounds most compelling to play?",
            "An introspective individual skilled at observation, deduction, and navigating moral grey areas.", // A: Bladerunner
            "A capable hero defined by their class, skills, and role within an adventuring party (even a small one).", // B: Pathfinder 2
            "A resilient survivor, defined more by their actions, experiences, and sworn oaths than by a class.", // C: Ironsworn
            "A stylish and skilled operator (merc, netrunner, etc.) carving out a niche in a dangerous world.", // D: Cyberpunk: RED
            "A highly skilled specialist defined by their areas of knowledge and expertise, capable of dissecting problems methodically.", // E: GUMSHOE
        ],
        [
            // Q6: Desired Atmosphere
            "What kind of overall atmosphere or feeling would you like our game sessions to have?",
            "Intriguing and suspenseful, with a sense of mystery and philosophical questions.", // A: Bladerunner
            "Exciting and adventurous, with a feeling of heroism and discovery in a magical world.", // B: Pathfinder 2
            "Immersive and personal, focused on the character's struggles, journeys, and the weight of their promises.", // C: Ironsworn
            "Intense and energetic, with high stakes, cool moments, and a rebellious attitude.", // D: Cyberpunk: RED
            "Tense and cerebral, building suspense as layers of the mystery are peeled back through careful investigation.", // E: GUMSHOE
        ],
        [
            // Q7: Problem-Solving Approach
            "When faced with difficulties, what approach sounds most appealing?",
            "Using intellect, empathy, and investigation to find nuanced or non-violent solutions.", // A: Bladerunner
            "Employing teamwork, tactics, and special abilities to overcome challenges directly.", // B: Pathfinder 2
            "Relying on resourcefulness, determination, and tough choices to endure and succeed.", // C: Ironsworn
            "Utilizing street smarts, specialized skills (combat, tech, social), and decisive action.", // D: Cyberpunk: RED
            "Applying specialized knowledge systematically to guarantee finding core information, then spending resources strategically for advantages.", // E: GUMSHOE
        ],
        [
            // Q8: NPC Interaction Style
            "What kind of interactions with non-player characters (NPCs) are most important to you?",
            "Deep, complex relationships and dialogues that reveal character and plot intricacies.", // A: Bladerunner
            "Meeting memorable allies, patrons, and villains who provide quests and challenges.", // B: Pathfinder 2
            "Encounters driven by the needs of survival, trust, and the harsh realities of the world.", // C: Ironsworn
            "Building a network of contacts, fixers, rivals, and clients essential for getting things done.", // D: Cyberpunk: RED
            "Interrogating sources, leveraging contacts, and analyzing testimony to extract crucial information and identify deception.", // E: GUMSHOE
        ],
        [
            // Q9: Character Creation Emphasis
            "What aspect of creating a character excites you most?",
            "Defining their personality, background, internal conflicts, and place in the world.", // A: Bladerunner
            "Choosing from many classes, feats, spells, and gear to build a mechanically unique hero.", // B: Pathfinder 2
            "Starting with a simple concept and discovering the character through their experiences and vows.", // C: Ironsworn
            "Crafting a unique style, role (like Netrunner, Solo), and acquiring cool gear/cybernetics.", // D: Cyberpunk: RED
            "Assigning points to specific investigative skills, defining *what* my character knows and is good at finding out.", // E: GUMSHOE
        ],
        [
            // Q10: Preferred Pace
            "What kind of pace do you generally prefer in games or stories?",
            "A more deliberate pace with time for investigation, reflection, and character interaction.", // A: Bladerunner
            "A varied pace mixing exploration, strategic combat, social encounters, and downtime.", // B: Pathfinder 2
            "A pace that flows organically with the journey and the challenges sworn upon.", // C: Ironsworn
            "A generally fast-paced experience driven by missions, action, and dealing with consequences.", // D: Cyberpunk: RED
            "A pace dictated by the flow of clues and investigation, often methodical but punctuated by discoveries or danger.", // E: GUMSHOE
        ],
        [
            // Q11: Tech vs Magic (or focus within Tech)
            "Which element sounds more exciting to incorporate into your character and the world?",
            "Grounded, near-future tech like advanced forensics, replicants, spinners (flying cars).", // A: Bladerunner
            "High fantasy magic: spellcasting, enchanted items, divine powers, mythical beasts.", // B: Pathfinder 2
            "A gritty world with little or no magic, or perhaps only subtle, mysterious rituals.", // C: Ironsworn
            "Transhuman tech: cybernetic limbs, neural interfaces, high-tech weapons, netrunning.", // D: Cyberpunk: RED
            "Using specific technology (forensics, hacking tools, surveillance gear) as tools to uncover information and solve mysteries.", // E: GUMSHOE
        ],
        [
            // Q12: Duet Focus
            "In a game with just you and the GM, what dynamic sounds best?",
            "A focused narrative on my character's cases, moral dilemmas, and personal stakes.", // A: Bladerunner
            "My character as the primary hero, perhaps with key NPC allies joining the adventures.", // B: Pathfinder 2 (adapting party)
            "A deeply personal story centered on my character's solitary journey and sworn quests.", // C: Ironsworn (strong duet fit)
            "My character navigating dangerous gigs and relationships within a vibrant, often hostile, city.", // D: Cyberpunk: RED
            "My character as the primary investigator, relying on their specific skills to drive the plot forward by uncovering the necessary clues.", // E: GUMSHOE
        ],
    ];

    // How to use:
    // Present each question (index 0) and its corresponding options (indices 1-5) to your SO.
    // Keep track of which letter (A, B, C, D, or E) they pick for each question.
    // Tally the results:
    // Mostly A -> Leaning Bladerunner
    // Mostly B -> Leaning Pathfinder 2 (but remember duet suitability issues)
    // Mostly C -> Leaning Ironsworn (needs cyberpunk re-skinning)
    // Mostly D -> Leaning Cyberpunk: RED (needs duet adaptation)
    // Mostly E -> Leaning GUMSHOE (needs cyberpunk setting implementation - e.g., Gaia Complex, Ashen Stars adaptation, or homebrew)
    // Mixed results -> Discuss the specific answers that appealed most to find the best compromise or hybrid approach.
    for question in choices {
        let choice = get_user_choice(question);
        match choice {
            1 => bladerunner += 1,
            2 => pathfinder += 1,
            3 => ironsworn += 1,
            4 => cyberpunk += 1,
            5 => gumshoe += 1,
            _ => unreachable!(),
        }
        println!("You chose option {}!\n", choice);
    }

    let systems = [
        ("Blade Runner RPG", bladerunner),
        ("Pathfinder", pathfinder),
        ("Ironsworn", ironsworn),
        ("Cyberpunk RED", cyberpunk),
        ("GUMSHOE", gumshoe),
    ];

    let (max_rpg, max_score) = systems
        .iter()
        .max_by_key(|&(_, score)| score)
        .map(|&(name, score)| (name, score)) // Convert Option<&(&str, i32)> to Option<(&str, i32)>
        .expect("System list should not be empty");

    println!(
        "Based on your choices, {} is the best fit with a score of {}!",
        max_rpg, max_score
    );
}
