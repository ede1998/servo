#[cfg(test)]
mod tests {
    use embedder_traits::resources::{self, Resource};
    use publicsuffix::{Domain, List, Psl, Suffix};

    #[test]
    fn test_compare_public_suffix_crate() {
        // the official list can be found at
        // https://publicsuffix.org/list/public_suffix_list.dat
        let list: List = include_str!("../../../public_suffix_list.dat")
            .parse()
            .unwrap();
        let test_cases = [
            ("example.com", ("com", "example.com")),
            ("sub.example.com", ("com", "example.com")),
            ("sub.sub.example.com", ("com", "example.com")),
            ("example.com.", ("com.", "example.com.")),
            ("sub.example.com.", ("com.", "example.com.")),
            ("sub.sub.example.com", ("com.", "example.com.")),
            ("bbc.co.uk", ("co.uk", "bbc.co.uk")),
            ("bbc.co.uk.", ("co.uk.", "bbc.co.uk.")),
            ("com", ("", "")),
            ("terminator", ("", "")),
            (".com", ("", "")),
            ("com.", ("", "")),
            ("", ("", "")),
            ("/////", ("", "")),
            ("#$%^&*():_!@", ("", "")),
            // ("example.no_tld", ("", "")),
            // (".......", ("", "")),
            ("https://www.example.com/example?example=a&b=3", ("", "")),
        ];

        for (input, (public, registrable)) in test_cases {
            let domain = print_domain(list.domain(input.as_bytes()))
                .unwrap_or_else(|| Domain::new(b"", Suffix::new(b"", None)));
            assert_eq!(domain.suffix(), public);
            assert_eq!(domain, registrable);
        }
        // // assert_eq!(suffix.typ(), Some(Type::Icann));

        // let domain = list.domain(b"www.example.com").unwrap();
        // assert_eq!(domain, "example.com");
        // assert_eq!(domain.suffix(), "com");

        // let domain = list.domain("www.食狮.中国".as_bytes()).unwrap();
        // assert_eq!(domain, "食狮.中国");
        // assert_eq!(domain.suffix(), "中国");

        // let domain = list.domain(b"www.xn--85x722f.xn--55qx5d.cn").unwrap();
        // assert_eq!(domain, "xn--85x722f.xn--55qx5d.cn");
        // assert_eq!(domain.suffix(), "xn--55qx5d.cn");

        // let domain = list.domain(b"a.b.example.uk.com").unwrap();
        // assert_eq!(domain, "example.uk.com");
        // assert_eq!(domain.suffix(), "uk.com");

        // let domain = list.domain(b"_tcp.example.com.").unwrap();
        // assert_eq!(domain, "example.com.");
        // assert_eq!(domain.suffix(), "com.");
    }

    fn print_domain<'a>(d: Option<Domain<'a>>) -> Option<Domain<'a>> {
        match d {
            Some(d) => println!(
                "Domain {{ full: {}, suffix: {}, is_public_suffix: {} }}",
                String::from_utf8_lossy(d.as_bytes()),
                String::from_utf8_lossy(d.suffix().as_bytes()),
                d.suffix().is_known(),
            ),
            None => println!("None"),
        }
        d
    }
}

/*
#[test]
fn sdf() {
    dbg!(PUB_DOMAINS.registrable_suffix("www.bbc.co.uk")); // bbc.co.uk
    dbg!(PUB_DOMAINS.registrable_suffix("www.bbc.co.uk.")); // bbc.co.uk.
    dbg!(PUB_DOMAINS.registrable_suffix("www.asdf.bbc.co.uk")); // bbc.co.uk
    dbg!(PUB_DOMAINS.registrable_suffix("www.asdf.bbc.co.uk.")); // bbc.co.uk.
    dbg!(PUB_DOMAINS.registrable_suffix("googleapis.com")); // googleapis.com
    dbg!(PUB_DOMAINS.registrable_suffix("googleapis.com.")); // googleapis.com.
    dbg!(PUB_DOMAINS.registrable_suffix("ede1998.googleapis.com")); // ede1998.googleapis.com
    dbg!(PUB_DOMAINS.registrable_suffix("ede1998.googleapis.com.")); // ede1998.googleapis.com.
    dbg!(PUB_DOMAINS.registrable_suffix("asdf.ede1998.googleapis.com")); // ede1998.googleapis.com
    dbg!(PUB_DOMAINS.registrable_suffix("asdf.ede1998.googleapis.com.")); // ede1998.googleapis.com.

    dbg!(PUB_DOMAINS.public_suffix("www.bbc.co.uk")); // co.uk
    dbg!(PUB_DOMAINS.public_suffix("www.bbc.co.uk.")); // co.uk.
    dbg!(PUB_DOMAINS.public_suffix("www.asdf.bbc.co.uk")); // co.uk
    dbg!(PUB_DOMAINS.public_suffix("www.asdf.bbc.co.uk.")); // co.uk.
    dbg!(PUB_DOMAINS.public_suffix("googleapis.com")); // googleapis.com
    dbg!(PUB_DOMAINS.public_suffix("googleapis.com.")); // googleapis.com.
    dbg!(PUB_DOMAINS.public_suffix("ede1998.googleapis.com")); // googleapis.com
    dbg!(PUB_DOMAINS.public_suffix("ede1998.googleapis.com.")); // googleapis.com.
    dbg!(PUB_DOMAINS.public_suffix("asdf.ede1998.googleapis.com")); // googleapis.com
    dbg!(PUB_DOMAINS.public_suffix("asdf.ede1998.googleapis.com.")); // googleapis.com.

    assert_eq!("", PUB_DOMAINS.registrable_suffix("www.bbc.co.uk"));
}

 */
