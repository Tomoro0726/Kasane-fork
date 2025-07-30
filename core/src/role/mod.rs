use std::collections::{HashMap, HashSet};

use crate::io::Space;

struct User {
    name: String,
    auth: HashSet<Auth>,
    permission: HashMap<Space, HashSet<Permission>>,
}

enum Auth {
    Password(String),
}

enum Permission {
    //get
    Read,

    //showkeys,put,set,delete
    Write,

    //showskeys,addkey,deletekey
    Admin,
}
