use ldap3::*;

use ldap3::result::Result;
use std::{process::exit, vec};

fn main() {
    let ldap = LdapConn::new("ldap://192.168.0.100:3268");

    let mut ldapcon = match ldap {
        Ok(l) => l,
        Err(r) => panic!("{}", r),
    };

    ldapcon
        .simple_bind("CN=Administrator,CN=Users,DC=tech69,DC=local", "Passw0rd")
        .unwrap();

    let res = ldapcon
        .search(
            "DC=tech69,DC=local",
            Scope::Subtree,
            "(objectclass=user)",
            vec!["dn"],
        )
        .unwrap();

    let (re, ldapresult) = res.success().unwrap();

    for i in re {
        println!("{:?}", SearchEntry::construct(i))
    }
}
