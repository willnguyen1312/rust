use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Team {
    name: String,
    wins: u32,
    points: u32,
    draw: u32,
    losses: u32,
    mp: u32,
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            wins: 0,
            points: 0,
            draw: 0,
            losses: 0,
            mp: 0,
        }
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
        self.points += 3;
        self.mp += 1;
    }

    pub fn add_draw(&mut self) {
        self.draw += 1;
        self.points += 1;
        self.mp += 1;
    }

    pub fn add_loss(&mut self) {
        self.losses += 1;
        self.mp += 1;
    }

    pub fn print_score(&self) -> String {
        format!(
            "{:width1$}| {:width2$} | {:width2$} | {:width2$} | {:width2$} | {:width2$}",
            self.name,
            self.mp,
            self.wins,
            self.draw,
            self.losses,
            self.points,
            width1 = 31,
            width2 = 2
        )
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.points < other.points || (self.points == other.points && self.name > other.name) {
            return Ordering::Greater;
        } else if self.points > other.points
            || (self.points == other.points && self.name < other.name)
        {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        (self.points, &self.name) == (other.points, &other.name)
    }
}

impl Eq for Team {}

pub fn print_header() -> String {
    format!("{:width1$}| MP |  W |  D |  L |  P", "Team", width1 = 31)
}

pub fn tally(match_results: &str) -> String {
    let mut teams = HashMap::<String, Team>::new();

    // update scores
    for game in match_results.split('\n') {
        let v: Vec<&str> = game.split(';').collect();

        if v.len() != 3 {
            return print_header();
        }

        match v[2] {
            "win" => {
                let team1 = teams
                    .entry(v[0].to_string())
                    .or_insert_with(|| Team::new(v[0].to_string()));
                team1.add_win();
                let team2 = teams
                    .entry(v[1].to_string())
                    .or_insert_with(|| Team::new(v[1].to_string()));
                team2.add_loss();
            }
            "draw" => {
                let team1 = teams
                    .entry(v[0].to_string())
                    .or_insert_with(|| Team::new(v[0].to_string()));
                team1.add_draw();
                let team2 = teams
                    .entry(v[1].to_string())
                    .or_insert_with(|| Team::new(v[1].to_string()));
                team2.add_draw();
            }
            "loss" => {
                let team2 = teams
                    .entry(v[1].to_string())
                    .or_insert_with(|| Team::new(v[1].to_string()));
                team2.add_win();
                let team1 = teams
                    .entry(v[0].to_string())
                    .or_insert_with(|| Team::new(v[0].to_string()));
                team1.add_loss();
            }
            _ => return print_header(),
        }
    }

    //sort scores
    let mut v = Vec::<&Team>::new();
    for key in teams.keys() {
        v.push(teams.get(&key.to_string()).unwrap());
    }
    v.sort();

    // Print Scores
    let mut scores = print_header();
    for team in v {
        scores += "\n";
        scores += &team.print_score();
    }

    scores
}
