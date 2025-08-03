use std::collections::{HashMap, HashSet};

use crate::{
    io::{Key, Space},
    parser::Command,
};

struct User<'a> {
    name: String,
    auth: HashSet<Auth>,
    permission: HashMap<Command, Option<CommandTarget<'a>>>,
}

enum Auth {
    Password(String),
}

enum CommandTarget<'a> {
    Key(HashSet<&'a Key>),
    Space(HashSet<&'a Space>),
}
