use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use clap::Parser;

/// searches for patterns
#[derive(Parser, Debug)]
#[command(version = "0.1")]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// File to search
    input: String,
}

fn main() {

    // let one = [1, 2, 3];
    // let two: [u8; 3] = [1, 2, 3];
    // let blank1 = [0; 3];
    // let blank2: [u8; 3] = [0; 3];
    //
    // let arrays = [one, two, blank1, blank2];
    //
    // for a in &arrays {
    //     print!("{:?}: ", a);
    //     for n in a.iter() {
    //         print!("\t{} + 10 = {}", n, n+10);
    //     }
    //
    //     let mut sum = 0;
    //     for i in 0..a.len() {
    //         sum += a[i];
    //     }
    //     println!("\t(Σ{:?} = {})", a, sum);
    // }

/*
5 let args = App::new("grep-lite")
6 .version("0.1")
7 .about("searches for patterns")
8 .arg(Arg::with_name("pattern")
9 .help("The pattern to search for")
10 .takes_value(true)
11 .required(true))
12 .get_matches();
 */

    let args = Args::parse();

    let needle = args.pattern.as_str();

    let re = Regex::new(needle).unwrap();

    let ctx_lines = 2;

    let input = args.input.as_str();

    let f = File::open(input).unwrap();

    let reader = BufReader::new(f);

//     let haystack = "\
// Every face, every shop,
// bedroom window, public-house, and
// dark square is a picture
// feverishly turned--in search of what?
// It is the same with books.
// What do we seek
// through millions of pages?";

    let mut tags = vec![];

    let mut ctx = vec![];

    for (i, line) in reader.lines().enumerate() {
        if let Some(_) = re.find(line.unwrap().as_str()) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    let f = File::open(input).unwrap();

    let reader = BufReader::new(f);

    for (i, line_) in reader.lines().enumerate() {

        let line = line_.unwrap();

        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if i >= lower_bound && i <= upper_bound {
                let local_ctx = (i, line.clone());
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
