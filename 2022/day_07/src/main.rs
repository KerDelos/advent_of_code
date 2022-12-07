use nom::{
    IResult,
    combinator::{map, map_res},
    character::complete::{alpha1, digit1, char, line_ending},
    bytes::complete::tag,
    sequence::{tuple, pair, separated_pair, preceded},
    branch::alt,
    multi::separated_list1,
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
pub enum FileSystemEntry{
    File(FileEntry),
    Folder(FolderEntry)
}

impl FileSystemEntry{
    fn parse(input: &str) -> IResult<&str,Self>
    {
        let parse_file_entry = map(FileEntry::parse, |f| FileSystemEntry::File(f));
        let parse_folder_entry = map(FolderEntry::parse, |f| FileSystemEntry::Folder(f));
        alt((parse_file_entry,parse_folder_entry))(input)
    }
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

#[derive(Debug, Eq, PartialEq)]
pub struct LSCommand{
    output: Vec<FileSystemEntry>,
}

impl LSCommand{
    fn parse(input: &str) -> IResult<&str,Self>
    {
        let parse_call = tag("$ ls");
        let parse_output = separated_list1(line_ending, FileSystemEntry::parse);
        let parse_full = preceded(pair(parse_call, line_ending), parse_output);
        map_res(parse_full, |output| Ok::<LSCommand,Error>(LSCommand{output}))(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ShellCommand{
    LS(LSCommand),
    CD(CDCommand),
}

impl ShellCommand{
    fn parse(input: &str) -> IResult<&str,Self>
    {
        let parse_cd_command = map(CDCommand::parse, |f| ShellCommand::CD(f));
        let parse_ls_command = map(LSCommand::parse, |f| ShellCommand::LS(f));
        alt((parse_cd_command,parse_ls_command))(input)
    }
}

fn main() {
    let content = std::fs::read_to_string("src/input_0.txt").expect("can't read file");

    let mut parse_shell_output = separated_list1(line_ending,ShellCommand::parse);

    let shell_output = parse_shell_output(&content[..]);

    println!("{:?}", shell_output.unwrap().1);

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
    fn file_system_entry(){
        assert_eq!(FileSystemEntry::parse("29116 test").unwrap().1,FileSystemEntry::File(FileEntry{name: "test".to_owned(), size:  29116}));
        assert_eq!(FileSystemEntry::parse("dir ayygahjvsef").unwrap().1,FileSystemEntry::Folder(FolderEntry{name: "ayygahjvsef".to_owned()}));
    }

    #[test]
    fn cd_command(){
        assert_eq!(CDCommand::parse("$ cd /").unwrap().1, CDCommand::Root);
        assert_eq!(CDCommand::parse("$ cd ..").unwrap().1, CDCommand::Up);
        assert_eq!(CDCommand::parse("$ cd testfolder").unwrap().1,CDCommand::Folder("testfolder".to_owned()));
    }

    #[test]
    fn ls_command(){
        let input = "$ ls\ndir ayygahjvsef\n62596 h.lst";
        let expected = LSCommand{
            output: vec![
                FileSystemEntry::Folder(FolderEntry{name: "ayygahjvsef".to_owned()}),
                FileSystemEntry::File(FileEntry{name: "h.lst".to_owned(), size:  62596}),
                ]
        };
        assert_eq!(LSCommand::parse(input).unwrap().1, expected);
    }
}