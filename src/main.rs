use walkdir::WalkDir;
use dirty_comments::parser;

fn main() {
    for entry in WalkDir::new("./test_proj") {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            parser::parse(&entry.path()).unwrap();
        }
    }

    
    // let re = Regex::new("!(?P<tag_v>nco)-(?P<version>[a-z, 0-9]+)|!(?P<tag>nco)").unwrap();
    // if let Some(caps) = re.captures("# !nco-asdag12324") {
    //     println!("{}", caps.get(2).unwrap().as_str());
    // }

}