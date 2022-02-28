use array2d::Array2D;
use std::{collections::HashMap, fs::File, io::Write, time::Instant};
use rayon::prelude::*;
use ordered_float::NotNan;
use clap::Parser;

const NOT_IT: u8 = 0;
const GREEN_ALL: Word = [b'g'; 5];
type Word = [u8; 5];
type Colors = [u8; 5];

fn to_word(w: impl AsRef<[u8]>) -> Word {
    w.as_ref().try_into().unwrap()
}

struct Tree {
    guess: Word,
    total_guesses: usize,
    max_guesses: usize,
    children: HashMap<Colors, Tree>,
}

impl Tree {
    fn leaf(guess: Word) -> Self {
        Self {guess, total_guesses: 1, max_guesses: 1, children: Default::default(),}
    }
    fn print(&self, n_answers: usize) {
        println!(
            "{}, total guesses: {}, avg guesses: {}, max guesses: {}",
            std::str::from_utf8(&self.guess).unwrap(),
            self.total_guesses,
            self.total_guesses as f32 / n_answers as f32,
            self.max_guesses,
        )
    }
    fn write(&self, w: &mut impl Write, mut line: Vec<u8>) -> std::io::Result<()> {
        if self.children.is_empty() {
            line.extend(self.guess);
            line.push(b'\n');
            w.write_all(&line)
        } else {
            line.extend(self.guess);
            line.push(b',');
            for child in self.children.values() {
                child.write(w, line.clone())?;
            }
            Ok(())
        }
    }
}

fn score(mut guess: Word, mut answer: Word) -> Word {
    let mut colors = [b'b'; 5];

    for i in 0..5 {
        if guess[i] == answer[i] {
            colors[i] = b'g';
            answer[i] = NOT_IT;
            guess[i] = NOT_IT;
        }
    }

    for i in 0..5 {
        if guess[i] != NOT_IT {
            if let Some(j) = answer.iter().copied().position(|a| a == guess[i]) {
                colors[i] = b'y';
                answer[j] = NOT_IT;
                guess[i] = NOT_IT;
            }
        }
    }
    colors
}

fn wide_putin(
    params: &Params, 
    depth: usize,
    guesses: &[usize],
    answers: &[usize],
    guess_words: &[Word],
    answer_words: &[Word],
    mat: &Array2D<Word>
) -> Option<Tree> {
    assert!(!answers.is_empty());
    if depth >= 7 { return None; }
    if let &[single_ans] = answers { 
        return Some(Tree::leaf(answer_words[single_ans])); 
    }

    let opt_guesses: Vec<usize> = if depth == 0 && params.starting_word.is_some() {
        let starter: Word = to_word(params.starting_word.as_ref().unwrap());
        let starter_index: usize = guess_words.iter().position(|&w| w == starter).unwrap();
        vec![starter_index,]
    } else {
        let mut groups = HashMap::<Word, usize>::new();
        let mut ranked_guesses: Vec<(_, usize)> = guesses
            .iter()
            .map(|&guess| {
                groups.clear();
                for &answer in answers {
                    let colors = mat[(guess, answer)];
                    *groups.entry(colors).or_default() += 1;
                }

                let mut sum: usize = groups.values().copied().sum();
                sum -= groups.get(&GREEN_ALL).copied().unwrap_or_default();

                let avg = NotNan::new(sum as f64 / groups.len() as f64).unwrap();
                (avg, guess)
            }).collect();
        ranked_guesses.sort_unstable();
        ranked_guesses.iter().map(|(_score, guess)| *guess).take(params.n_guesses).collect()
    };

    assert!(!opt_guesses.is_empty());

    let tree = opt_guesses
        .iter()
        .filter_map(|&guess| {
            if depth == 0 {
                let guess_word = guess_words[guess];
                print!("Computing decision tree for starting word {}...\n", std::str::from_utf8(&guess_word).unwrap());
                std::io::stdout().flush().unwrap();
            }

            let mut tree = Tree {
                total_guesses: answers.len(),
                max_guesses: 0,
                guess: guess_words[guess],
                children: Default::default(),
            };

            let mut groups = HashMap::<Word, Vec<usize>>::new();
            for &answer in answers {
                let colors = mat[(guess, answer)];
                groups.entry(colors).or_default().push(answer);
            }

            let re = |(_score, grouped_answers): (&Colors, &Vec<usize>)| {
                wide_putin(params, depth + 1, guesses, grouped_answers, guess_words, answer_words, mat)
            };

            let children: Vec<Option<Tree>> = if depth <= 1 { groups.par_iter().map(re).collect() } else { groups.iter().map(re).collect() };
            for (&score, child) in groups.keys().zip(children) {
                let child = child?;
                if score != GREEN_ALL {
                    tree.total_guesses += child.total_guesses;
                }
                tree.max_guesses = tree.max_guesses.max(child.max_guesses + 1);
                tree.children.insert(score, child);
            }

            if depth == 0 { tree.print(answers.len()) }
            Some(tree)
        }).min_by_key(|tree| tree.total_guesses)?;
    Some(tree)
}

#[derive(Parser)]
struct Params {
    #[clap(short, long, default_value = "20")]
    n_guesses: usize,
    #[clap(long)]
    answers_only: bool,
    #[clap(long)]
    starting_word: Option<String>,
}
fn solve_matrix(guesses: &[Word], answers: &[Word]) -> Array2D<Word> {
    let time = Instant::now();
    print!("Now computing {}x{} matrix of 5-letter words...\n", guesses.len(), answers.len());
    std::io::stdout().flush().unwrap();

    let row: Vec<Vec<Word> > = guesses.iter().map(|guess| {
        let r = answers.iter().map(|answer| {
            let score = score(*guess, *answer);
            return score;
        }).collect();
        return r;
    }).collect();
    let mat = Array2D::from_rows(&row);
    print!("Process cleared. Time Elapsed: {:.2}s.\n", time.elapsed().as_secs());
    mat
}
fn main() {
    println!("Wordle Cheeser");
    static ACCEPTED: &[u8] = include_bytes!("../data/accepted.txt");
    static SOLUTIONS: &[u8] = include_bytes!("../data/solutions.txt");
    let params = Params::parse();
    let answers: Vec<Word> = SOLUTIONS.split(|&bool| bool == b'\n').map(to_word).collect();
    let mut guesses: Vec<Word> = answers.clone();
    if !params.answers_only {
        guesses.extend(ACCEPTED.split(|&b| b == b'\n').map(to_word));
        guesses.sort_unstable();
        guesses.dedup();
    }
    let mat = solve_matrix(&guesses, &answers);
    let g_indices: Vec<usize> = (0..guesses.len()).collect();
    let a_indices: Vec<usize> = (0..answers.len()).collect();
    let time = Instant::now();
    let decision_tree = wide_putin(&params, 0, &g_indices, &a_indices, &guesses, &answers, &mat).unwrap();
    print!("Finished computing decision tree within {:.2} seconds.\n", time.elapsed().as_secs());

    decision_tree.print(answers.len());

    let mut outfile = File::create("output.txt").unwrap();
    decision_tree.write(&mut outfile, vec![]).unwrap();
}
