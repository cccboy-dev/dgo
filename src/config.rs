use std::{
    collections::HashMap,
    fs::{read_to_string, write as fs_write}, path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // alias: Option<HashMap<String, String>>,
    alias: HashMap<String, Alias>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    path: PathBuf,
    level: u8,
}

#[derive(Debug)]
pub struct AliasVecItem {
    alias: String,
    path: PathBuf,
    level: u8,
}

pub struct Dgo {
    config_path: PathBuf,
    config: Config,
    aliases: Vec<AliasVecItem>,
}

impl Dgo {
    pub fn new(config_path: PathBuf) -> Dgo {
        let config = Dgo::load(&config_path);
        let mut dgo = Dgo {
            config_path,
            config,
            aliases: vec![],
        };

        dgo.sync_aliases();
        dgo
    }

    fn sync_aliases(&mut self) {
        let Dgo { config, .. } = self;
        let Config { alias } = config;

        let mut result: Vec<AliasVecItem> = alias
            .into_iter()
            .map(|(key, value)| {
                return AliasVecItem {
                    alias: key.clone(),
                    path: value.path.clone(),
                    level: value.level,
                };
            })
            .collect();

        result.sort_by(|a, b| b.level.cmp(&a.level));

        self.aliases = result;
    }

    fn load(config_path: &PathBuf) -> Config {
        let tome_str = read_to_string(config_path);
        if let Ok(tome_str) = tome_str {
            toml::from_str(&tome_str).expect("Unable to parse toml")
        } else {
            Config {
                alias: HashMap::new(),
            }
        }
    }

    fn resort_aliases(&mut self) {
        let Dgo { aliases, .. } = self;

        aliases.sort_by(|a, b| b.level.cmp(&a.level));
    }

    pub fn alias_list(&self) {
        let Dgo { aliases, .. } = self;

        println!("alias list: {:?}", aliases);
    }

    fn save(&mut self) {
        self.resort_aliases();

        let Dgo {
            config_path,
            config,
            aliases,
        } = self;

        config.alias.clear();

        aliases.iter().for_each(|alias| {
            config.alias.insert(
                alias.alias.clone(),
                Alias {
                    path: alias.path.clone(),
                    level: alias.level,
                },
            );
        });

        fs_write(config_path, toml::to_string(&config).unwrap()).expect("Unable to write file");

        println!("save successful!");
    }

    pub fn level_increase(&mut self, key: &str) -> () {
        let Dgo { aliases, .. } = self;

        let value = aliases.iter_mut().find(|value| value.alias == key).unwrap();

        value.level += 1;

        self.save()
    }

    pub fn add_alias(&mut self, key: &str, path: PathBuf) -> () {
        println!("add_alias: {:?} {:?}", key, path);

        let value_option = self.aliases.iter().find(|value| value.alias == key);
        if let Some(value) = value_option {
            if path == value.path {
                self.level_increase(key);
                return;
            }
            panic!("alias: {:?} is exists", key);
        }

        self.aliases.push(AliasVecItem {
            alias: key.into(),
            path: path.into(),
            level: 0,
        });

        self.save();
    }

    pub fn rm_alias(&mut self, key: &str) {
        let Dgo { aliases, .. } = self;

        let index = aliases.iter().position(|value| value.alias == key).unwrap();

        aliases.remove(index);

        self.save()
    }

    pub fn clear_alias(&mut self) {
        let Dgo { aliases, .. } = self;

        aliases.clear();

        self.save()
    }
}
