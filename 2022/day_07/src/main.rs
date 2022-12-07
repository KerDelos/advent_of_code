use nom::{
    IResult,
    combinator::{map, map_res},
    character::complete::{alpha1, digit1, char, line_ending},
    bytes::complete::tag,
    sequence::{tuple, pair, separated_pair, preceded},
    branch::alt,
    multi::separated_list1,
};
use std::{str::FromStr, fmt::Error, cell::RefCell, iter::Cloned, fs::File, vec};

pub fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

pub fn parse_alpha_to_string(input: &str) -> IResult<&str, String>
{
    map_res(alpha1, String::from_str)(input)
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileSystemEntry{
    File { name: String, size: i32},
    Folder { name: String, children: Vec<FileSystemEntry>}
}

impl FileSystemEntry{
    pub fn parse_filename_with_extensions(input: &str) -> IResult<&str, String>
    {
        map_res(
            tuple((parse_alpha_to_string,char('.'),parse_alpha_to_string)),
            |(name, sep, ext)| Ok::<String,Error>(format!("{}{}{}",name,sep,ext))
            )(input)
    }

    fn parse_file_entry(input: &str) -> IResult<&str,Self>
    {
        let filename_parser = alt((Self::parse_filename_with_extensions,parse_alpha_to_string));
        let size_and_name_parser = separated_pair(parse_number, char(' '), filename_parser);
        map(size_and_name_parser, |(size, name)| FileSystemEntry::File{name, size})(input)
    }

    fn parse_folder_entry(input: &str) -> IResult<&str,Self>
    {
        map(preceded(tag("dir "), parse_alpha_to_string), |name| FileSystemEntry::Folder{name, children: Vec::new()})(input)
    }

    fn parse(input: &str) -> IResult<&str,Self>
    {
        alt((Self::parse_file_entry,Self::parse_folder_entry))(input)
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

fn traverse_and_get_folder<'a>(path: Vec::<String>, root: &'a mut FileSystemEntry) -> Option<&'a mut FileSystemEntry>{
    if path.is_empty() {
        return Some(root);
    }
    let next_folder = &path[0];
    let remaining_path : Vec<String> = Vec::from_iter(path[1..].iter().cloned());
    if let FileSystemEntry::Folder { children, .. } = root {
        for child in children {
            if let FileSystemEntry::Folder { name, ..} = child {
                if name.as_str() == next_folder {
                    return traverse_and_get_folder(remaining_path, child);
                }
            }
        }
    }
    return None;
}

fn traverse_and_get_folder_smaller_than(max_size: i32, root: & FileSystemEntry) -> Option<((String,i32),Vec::<(String,i32)>)>{
    if let FileSystemEntry::Folder { name, children } = root {
        let total_file_size : i32 = children.iter().filter_map(
            |c| 
            match c { 
                FileSystemEntry::File {size, .. } => Some(*size),
                 _ => None
                }
            ).sum();
        let child_folders_info = children.iter().filter_map(|f| traverse_and_get_folder_smaller_than(max_size, f));
        let mut total_folders_size :i32 = 0;
        let mut small_folders : Vec::<(String,i32)> = Vec::new();
        for mut info in child_folders_info {
            small_folders.append(&mut info.1);
            total_folders_size += info.0.1;
            if info.0.1 < max_size {
                small_folders.push(info.0);
            }
        }
        return Some(((name.clone(), total_file_size+total_folders_size),small_folders))
    }
    None
}

fn main() {
    let content = std::fs::read_to_string("src/input_1.txt").expect("can't read file");

    let mut parse_shell_output = separated_list1(line_ending,ShellCommand::parse);

    let shell_output = parse_shell_output(&content[..]);

    //construct filesystem
    let mut filesystem_root = FileSystemEntry::Folder{name: "root".to_owned(), children: Vec::new()};
    let mut cwd = Vec::<String>::new();
    for command in shell_output.unwrap().1 {
        println!("{:?}", command);
        match command {
            ShellCommand::CD(cd) => match cd {
                CDCommand::Root => cwd.clear(),
                CDCommand::Up => _ = cwd.pop(),
                CDCommand::Folder(folder_name) => cwd.push(folder_name.clone()),
            },
            ShellCommand::LS(LSCommand { output }) => {
                let mut folder = traverse_and_get_folder(cwd.clone(), &mut filesystem_root).expect("folder doesn't exist");
                for mut entry in output{
                    if let FileSystemEntry::Folder{children,..} = folder{
                        children.push(entry)
                    }
                    else{
                        panic!("this should be a folder");
                    }
                }
            },
        }
    }

    //find answer to problem one
    let res1 = traverse_and_get_folder_smaller_than(100000, &filesystem_root).unwrap().1;
    println!("{:?}",res1);
    let sum1 :i32 = res1.iter().map(|(_, size)| size).sum();
    println!("Solution to problem 1 is {}",sum1);
}

//TODO didn't manage to have a "parent" ref for each folder and also having cwd as a ref to a folder
//this would have avoided goind through the entire filesystem for each ls command

//TODO didn't manage to make an equivalent to traverse_and_get_folder without using recursivity
// fn get_folder<'a>(path: Vec::<&str>, filesystem: &'a mut FileSystemEntry) -> &'a mut FileSystemEntry{
//     let mut cwd = filesystem; 
//     for step in path {
//         if let FileSystemEntry::Folder {children, ..} = cwd {
//             for child in children {
//                 if let FileSystemEntry::Folder { name, ..} = child {
//                     if name.as_str() == step {
//                         cwd = child;
//                     }
//                 }
//             }
//         }
//     }
//     return cwd;
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers()
    {
        assert_eq!(parse_alpha_to_string("test__").unwrap().1, "test");
        assert_eq!(parse_alpha_to_string("test__").unwrap().0, "__");
        assert_eq!(parse_number("15263").unwrap().1, 15263);
        assert_eq!(FileSystemEntry::parse_filename_with_extensions("test.txt").unwrap().1, "test.txt");
    }

    #[test]
    fn file_entry() {
        assert_eq!(FileSystemEntry::parse_file_entry("29116 test").unwrap().1, FileSystemEntry::File{name: "test".to_owned(), size:  29116});
        assert_eq!(FileSystemEntry::parse_file_entry("45 test.txt").unwrap().1, FileSystemEntry::File{name: "test.txt".to_owned(), size:  45});
    }

    #[test]
    fn folder_entry(){
        assert_eq!(FileSystemEntry::parse_folder_entry("dir ayygahjvsef").unwrap().1, FileSystemEntry::Folder{name: "ayygahjvsef".to_owned(), children: Vec::new()});
    }

    #[test]
    fn file_system_entry(){
        assert_eq!(FileSystemEntry::parse("29116 test").unwrap().1,FileSystemEntry::File{name: "test".to_owned(), size:  29116});
        assert_eq!(FileSystemEntry::parse("dir ayygahjvsef").unwrap().1,FileSystemEntry::Folder{name: "ayygahjvsef".to_owned(), children: Vec::new()});
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
                FileSystemEntry::Folder{name: "ayygahjvsef".to_owned(), children: Vec::new()},
                FileSystemEntry::File{name: "h.lst".to_owned(), size:  62596},
                ]
        };
        assert_eq!(LSCommand::parse(input).unwrap().1, expected);
    }
}