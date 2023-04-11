use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let mut output: String = String::with_capacity(1000);
    output += "\n\\begin{table}[ht]\n\\centering\n\\caption{CAPTION}\n\\begin{tabular}{";
    let mut reader = csv::Reader::from_path(args.input)?;

    let header = reader.headers()?;
    let len_header = header.len();
    let mut col_width = "p{".to_string() + &(1.0 / (len_header as f32 + 1.0)).to_string();
    col_width += "\\textwidth}";
    output += &col_width.repeat(header.len());
    output += "}\n\\toprule\n";

    let mut it = header.into_iter().peekable();
    while let Some(h) = it.next() {
        if it.peek().is_none() {
            output += h;
            output += " ";
        } else {
            output += h;
            output += " & ";
        }
    }
    output += "\\\\\n\\midrule\n";

    for line in reader.records().take(args.max_rows) {
        for (i, val) in line?.into_iter().enumerate() {
            if i == len_header - 1 {
                output += val;
                output += " \\\\\n";
            } else {
                output += val;
                output += " & ";
            }
        }
    }
    output += "\\bottomrule\n\\end{tabular}\n\\label{tab:ref}\n\\end{table}\n\n";
    print!("{}", output);
    Ok(())
}

#[derive(StructOpt, Debug)]
#[allow(unused_variables)]
#[structopt(
    name = ".csv to LaTeX table converter",
    version = "0.1.0",
    author = "Fraign Analytics LLC",
    about = "A command line tool for the conversion of a .csv file to a LaTeX table with some simple defaults."
)]
pub struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(short = "r", long, default_value = "20")]
    pub max_rows: usize,
    #[structopt(short = "h", long, default_value = "0")]
    pub header_row: usize,
    #[structopt(short = "d", long, default_value = ",")]
    pub delimiter: String,
}
