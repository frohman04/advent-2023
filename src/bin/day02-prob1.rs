use std::collections::HashMap;

fn main() {
    let games = std::fs::read_to_string("src/bin/day02.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Game::from_line(val.to_string()))
                .collect::<Vec<Game>>()
        })
        .expect("Unable to open file");
    let possible_games = games.into_iter().filter(|game| {
        is_possible_game(
            game,
            &Draw {
                red: 12,
                green: 13,
                blue: 14,
            },
        )
    });
    println!("{}", possible_games.map(|game| game.id).sum::<u16>());
}

fn is_possible_game(game: &Game, bag_contents: &Draw) -> bool {
    game.draws.iter().all(|draw| {
        draw.red <= bag_contents.red
            && draw.green <= bag_contents.green
            && draw.blue <= bag_contents.blue
    })
}

#[derive(Debug, PartialEq)]
struct Draw {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[derive(Debug, PartialEq)]
struct Game {
    pub id: u16,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn from_line(line: String) -> Game {
        let line = &line["Game ".len()..];

        let col_i = line
            .find(':')
            .unwrap_or_else(|| panic!("unable to find ':' in: {}", line));
        let game_id = line[..col_i]
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("unable to parse id: {}", &line[..col_i]));

        let line = &line[(col_i + 1)..];
        let draws = line
            .split(';')
            .map(|draw_str| {
                let draw_str = draw_str.trim();
                let colors = draw_str
                    .split(',')
                    .map(|color_str| {
                        let color_str = color_str.trim();
                        let bits = color_str.split(' ').collect::<Vec<&str>>();
                        let color = bits[1];
                        let count = bits[0].parse::<u16>().unwrap_or_else(|_| {
                            panic!("unable to parse count for color {}: {}", color, bits[0])
                        });
                        (color, count)
                    })
                    .collect::<HashMap<&str, u16>>();
                Draw {
                    red: *colors.get("red").unwrap_or(&0),
                    green: *colors.get("green").unwrap_or(&0),
                    blue: *colors.get("blue").unwrap_or(&0),
                }
            })
            .collect::<Vec<Draw>>();

        Game { id: game_id, draws }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_from_line() {
        assert_eq!(
            Game::from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game {
                id: 1,
                draws: vec![
                    Draw {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Draw {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Draw {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ]
            },
        )
    }

    #[test]
    fn test_is_possible_game_true() {
        assert_eq!(
            is_possible_game(
                &Game {
                    id: 1,
                    draws: vec![
                        Draw {
                            red: 4,
                            green: 0,
                            blue: 3,
                        },
                        Draw {
                            red: 1,
                            green: 2,
                            blue: 6,
                        },
                        Draw {
                            red: 0,
                            green: 2,
                            blue: 0,
                        },
                    ]
                },
                &Draw {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            ),
            true,
        )
    }

    #[test]
    fn test_is_possible_game_false() {
        assert_eq!(
            is_possible_game(
                &Game {
                    id: 1,
                    draws: vec![
                        Draw {
                            red: 20,
                            green: 8,
                            blue: 6,
                        },
                        Draw {
                            red: 4,
                            green: 13,
                            blue: 5,
                        },
                        Draw {
                            red: 1,
                            green: 5,
                            blue: 0,
                        },
                    ]
                },
                &Draw {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            ),
            false,
        )
    }
}
