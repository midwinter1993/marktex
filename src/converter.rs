use std::fs::File;
use std::io::{BufWriter, Write};

use pulldown_cmark::{Alignment, Event, Options, Parser, Tag};

pub struct Converter {
    writer: BufWriter<File>,

    current_nr_column: usize,
    current_id_column: usize,
}

impl Converter {
    pub fn new(output: &str) -> Self {
        let f = File::create(&output).expect("Unable to create file");
        Converter {
            writer: BufWriter::new(f),
            current_nr_column: 0,
            current_id_column: 0,
        }
    }

    fn start_tag(&mut self, tag: &Tag) -> Result<(), std::io::Error> {
        match tag {
            Tag::Paragraph => {},

            Tag::Heading(level) => match level {
                1 => write!(self.writer, "\\section{{")?,
                2 => write!(self.writer, "\\subsection{{")?,
                3 => write!(self.writer, "\\subsection{{")?,
                _ => panic!(),
            },

            Tag::BlockQuote => {}

            Tag::List(_) => write!(self.writer, "\\begin{{itemize}}\n")?,

            Tag::Item => {
                write!(self.writer, "\\item ")?;
            }

            Tag::Strong => {
                write!(self.writer, "\\textbf{{")?;
            }

            Tag::Link(_ty, url, title) => {
                write!(self.writer, "\\href{{{}}}{{{}}}", url, title)?;
            }

            Tag::Table(vec_align) => {
                self.current_nr_column = vec_align.len();

                writeln!(self.writer, "\\begin{{table}}[]")?;
                write!(self.writer, "\\begin{{tabular}}")?;
                write!(self.writer, "{{")?;
                for align in vec_align {
                    match align {
                        Alignment::None | Alignment::Left => write!(self.writer, "l")?,
                        Alignment::Center => write!(self.writer, "c")?,
                        Alignment::Right => write!(self.writer, "r")?,
                    }
                }
                write!(self.writer, "}}\n")?;
            }

            Tag::TableHead => {}

            Tag::TableRow => {
                self.current_id_column = 0;
                writeln!(self.writer, " \\\\")?;
            }

            Tag::TableCell => {}

            Tag::CodeBlock(_) => {
                writeln!(self.writer, "\\begin{{lstlisting}}")?;
            }

            _ => {
                writeln!(self.writer, "{:?}", tag)?;
                panic!();
            }
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: &Tag) -> Result<(), std::io::Error> {
        match tag {
            Tag::Paragraph => {
                writeln!(self.writer, "")?;
                writeln!(self.writer, "")?;
            }

            Tag::Heading(_) => write!(self.writer, "}}\n")?,

            Tag::BlockQuote => {}

            Tag::List(_) => write!(self.writer, "\\end{{itemize}}\n")?,

            Tag::Item => {
                writeln!(self.writer, "")?;
            }

            Tag::Strong => {
                write!(self.writer, "}}")?;
            }

            Tag::Link(_, _, _) => {}

            Tag::Table(_vec_align) => {
                self.current_nr_column = 0;

                writeln!(self.writer, "\n\\end{{tabular}}")?;
                writeln!(self.writer, "\\end{{table}}")?;
            }

            Tag::TableHead => {
                // writeln!(self.writer, "\\\\")?;
            }

            Tag::TableRow => {
                // writeln!(self.writer, "\\\\")?;
            }

            Tag::TableCell => {
                self.current_id_column += 1;

                if self.current_id_column < self.current_nr_column {
                    write!(self.writer, " & ")?;
                }
            }

            Tag::CodeBlock(_) => {
                writeln!(self.writer, "\\end{{lstlisting}}")?;
            }

            _ => {
                println!("{:?}", tag);
                panic!();
            }
        }
        Ok(())
    }

    pub fn parse_markdown(&mut self, markdown: String) -> Result<(), std::io::Error> {
        // Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        // Write to String buffer.
        // let mut html_output = String::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&markdown, options);

        for event in parser {
            println!("{:?}", event);
            match event {
                Event::Start(tag) => {
                    self.start_tag(&tag)?;
                }

                Event::End(tag) => {
                    self.end_tag(&tag)?;
                }

                Event::Text(text) => {
                    write!(self.writer, "{}", text)?;
                }

                Event::Code(code) => {
                    write!(self.writer, "\\lstinline[]${}$", code)?;
                }

                _ => {
                    println!("");
                    println!("{:?}", event);
                    panic!();
                }
            }
        }
        Ok(())
    }
}

