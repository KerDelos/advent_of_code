use nom::{
    IResult,
    combinator::{
        map,
        map_res,
    },
    character::complete::{
        space0,
        anychar,
        alpha1,
        digit1,
        char,
    },
    bytes::complete::tag,
    sequence::{
        tuple,
        separated_pair, preceded
    },
    branch::alt,
};
use std::{str::FromStr, fmt::Error};

pub fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

pub fn parse_alpha_to_string(input: &str) -> IResult<&str, String>
{
    map_res(alpha1, String::from_str)(input)
}

pub fn parse_filename(input: &str) -> IResult<&str, String>
{
    let complete_filename_parser = tuple((parse_alpha_to_string,char('.'),parse_alpha_to_string));

    map_res(
        complete_filename_parser,
         |(name, sep, ext)| Ok::<String,Error>(format!("{}{}{}",name,sep,ext)) 
        )(input)
}

#[derive(Debug, Eq, PartialEq)]
pub struct FileEntry {
    pub name: String,
    pub size: u32,
}

impl FileEntry {
    fn parse(input: &str) -> IResult<&str,Self>
    {
        let filename_parser = alt((parse_filename,parse_alpha_to_string));
        let size_and_name_parser = separated_pair(parse_number, char(' '), filename_parser);
        map(size_and_name_parser, |(size, name)| Self {name, size})(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FolderEntry {
    pub name: String,
}

impl FolderEntry {
    fn parse(input: &str) -> IResult<&str,Self>
    {
        map(preceded(tag("dir "), parse_alpha_to_string), |name| Self {name})(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CDCommand{
    Root,
    Up,
    Folder(String),
}
impl CDCommand{
    fn argument_parser(input: &str) -> IResult<&str,&str> {
        alt((tag("/"),tag(".."),alpha1))(input)
    }

    fn parse(input: &str) -> IResult<&str,Self>
    {
        map(preceded(tag("$ cd "), CDCommand::argument_parser), 
            |argument| 
            match argument {
                "/" => CDCommand::Root,
                ".." => CDCommand::Up,
                folder_name => CDCommand::Folder(folder_name.to_owned())
            }
        )(input)
    }
}

fn main() {
    let content = std::fs::read_to_string("src/input_0.txt").expect("can't read file");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers()
    {
        assert_eq!(parse_alpha_to_string("test__").unwrap().1, "test");
        assert_eq!(parse_alpha_to_string("test__").unwrap().0, "__");
        assert_eq!(parse_number("15263").unwrap().1, 15263);
        assert_eq!(parse_filename("test.txt").unwrap().1, "test.txt");
    }

    #[test]
    fn file_entry() {
        assert_eq!(FileEntry::parse("29116 test").unwrap().1, FileEntry{name: "test".to_owned(), size:  29116});
        assert_eq!(FileEntry::parse("45 test.txt").unwrap().1, FileEntry{name: "test.txt".to_owned(), size:  45});
    }

    #[test]
    fn folder_entry(){
        assert_eq!(FolderEntry::parse("dir ayygahjvsef").unwrap().1, FolderEntry{name: "ayygahjvsef".to_owned()});
    }

    #[test]
    fn cd_command(){
        assert_eq!(CDCommand::parse("$ cd /").unwrap().1, CDCommand::Root);
        assert_eq!(CDCommand::parse("$ cd ..").unwrap().1, CDCommand::Up);
        assert_eq!(CDCommand::parse("$ cd testfolder").unwrap().1,CDCommand::Folder("testfolder".to_owned()));
    }
}