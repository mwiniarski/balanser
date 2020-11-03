use std::io::prelude::*;
use std::fs::File;
use std::env;

extern crate yaml_rust;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

fn merge_hashes(mut left_hash: Hash, right_hash: Hash) -> Yaml {
    right_hash.into_iter().for_each(|(key, value)| {
        left_hash.insert(key, value);
    });
    return Yaml::Hash(left_hash);
}

fn merge_hash(left: Hash, right: Yaml) -> Yaml {
    match right {
        Yaml::Hash(right_hash) => merge_hashes(left, right_hash),
        _ => Yaml::Hash(left),
    }
}

fn merge_docs(doc: Yaml, right: Yaml) -> Yaml {
    match doc {
        Yaml::Hash(left_hash) => merge_hash(left_hash, right),
        _ => doc,
    }
}

fn remove_first_char(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

/// Loads the configuration (from the config.yaml file, cmd line args)
pub fn init() -> Yaml {
    // Load config from file
    let mut file = File::open("config.yaml").expect("Unable to open the config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the config file");
    let config = &YamlLoader::load_from_str(&contents).unwrap()[0];

    // Load config from cmd line args
    let args: Vec<String> = env::args().collect();
    let mut args_hash_map = Hash::new();
    for arg in args {
        if arg.contains("=") {
            // key=value
            let (key, value) = arg.split_at(arg.find('=').unwrap());
            args_hash_map.insert(Yaml::from_str(key), Yaml::from_str(remove_first_char(&value).unwrap()));
        }
        else {
            // key=true
            args_hash_map.insert(Yaml::from_str(&arg), Yaml::Boolean(true));
        }
    }
    let args_hash = Yaml::Hash(args_hash_map);
    let config_args = merge_docs(config.clone(), args_hash);

    return config_args;
}
