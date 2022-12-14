use crate::ReadableRe::{Period, Raw};
use crate::{chars, either, exactly, group, one_or_more, ranged, starts_and_ends_with, ReadableRe};
use once_cell::sync::Lazy;

const U8: Lazy<ReadableRe> = Lazy::new(|| {
    group(either([
        chars("0-9"),
        chars("1-9") + chars("0-9"),
        Raw("1") + exactly(2, chars("0-9")),
        Raw("2") + chars("0-4") + chars("0-9"),
        Raw("25") + chars("0-5"),
    ]))
});

/// IPV4 address
pub const IPV4_ADDRESS: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(group(
        U8.clone() + ".".into() + U8.clone() + ".".into() + U8.clone() + ".".into() + U8.clone(),
    ))
});

/// IPV6 address, found [here](https://regex101.com/r/uwPxJf/1)
pub const IPV6_ADDRESS: Lazy<ReadableRe> = Lazy::new(|| {
    // well... [`Raw`] is there to be used too...
    Raw(
        r"(?i)(?:[\da-f]{0,4}:){1,7}(?:(?P<ipv4>(?:(?:25[0-5]|2[0-4]\d|1?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|1?\d\d?))|[\da-f]{0,4})",
    )
});

/// URL, found [here]()
pub const SIMPLE_URL: ReadableRe = Raw(
    r"(https?://)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#()?&//=]*)",
);

/// Email as per [W3C HTML5 specification](https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address)
pub const EMAIL: ReadableRe =
    // we trust that specification, no need to build a monster here when we already have one
    Raw(
        r"^[a-zA-Z0-9.!#$%&'*+\\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$",
    );

pub const SIMPLE_EMAIL: Lazy<ReadableRe> = Lazy::new(|| {
    starts_and_ends_with(
        group(one_or_more(chars(r"a-z0-9_\.\+-")))
            + "@".into()
            + group(one_or_more(chars(r"\da-z\.-")))
            + Period
            + group(ranged(2..6, chars(r"a-z\."))),
    )
});

#[cfg(test)]
mod tests {
    use crate::presets::network::{
        EMAIL, IPV4_ADDRESS, IPV6_ADDRESS, SIMPLE_EMAIL, SIMPLE_URL, U8,
    };
    use crate::starts_and_ends_with;

    #[test]
    fn u8_re() {
        let query = starts_and_ends_with(U8.clone()).compile().unwrap();
        for i in 0..256 {
            assert!(query.is_match(&i.to_string()), "Failed to match: {}", i);
        }
        assert!(!query.is_match("256"));
    }

    #[test]
    fn ipv4() {
        let query = IPV4_ADDRESS.compile().unwrap();
        for v in 0..256 {
            let s = format!("{0}.{0}.{0}.{0}", v);
            assert!(query.is_match(&s), "Failed to match: {}", s);
        }
    }

    #[test]
    fn ipv6() {
        let ips = [
            "2001:db8:3333:4444:5555:6666:7777:8888",
            "2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF",
            "2001:0db8:0001:0000:0000:0ab9:C0A8:",
            "fffe:3465:efab:23fe:2235:6565",
            "2001:0000:6dcd:8c74:::ac32:6a1",
            "2345:0425:2CA1:::5673:23b5",
            "2001:db8:1::ab9:C0A8:102",
            "2001:db8::1234:5678",
            "FF02:0:0:0:0:0:0:2",
            "fdf8:f53b:82e4::53",
            "fe80::200:5aee:feaa:20a2",
            "2001:0000:4136:e378:",
            "8000:63bf:3fff:fdd2",
            "2001:db8::",
            "::1234:5678",
            "2000::",
            "2001:db8:a0b:12f0::1",
            "2001:4:112:cd:65a:753:0:a1",
            "2001:0002:6c::430",
            "2001:5::",
            "fe08::7:8",
            "2001:10:240:ab::a",
            "2002:cb0a:3cdd:1::1",
            "2001:db8:8:4::2",
            "ff01:0:0:0:0:0:0:2",
            "1:2:3:4:5:6:7:8",
            "1::3:4:5:6:7:8",
            "1::4:5:6:7:8",
            "1:2:3::5:6:7:8",
            "1::7:8",
            "1:2:3:4:5::7:8",
            "1:2:3:4:5::8",
            "::2:3:4:5:6:7:8",
            "::8",
            "::",
            "::1",
            ":1",
            "::ffff:192.0.2.47",
            "::255.255.255.255",
            "::ffff:255.255.255.255",
            "::ffff:0:255.255.255.255",
            "2001:db8:3:4::192.0.2.33",
            "64:ff9b::192.0.2.33",
            "2001:db8:3333:4444:5555:6666:1.2.3.4",
            "24a6:57:c:36cf:0000:5efe:109.205.140.116",
            "::11.22.33.44",
            "::10.0.0.3",
            "2001:db8::123.123.123.123",
            "::ffff:10.0.0.3",
            "::1234:5678:91.123.4.56",
            "::1234:5678:1.2.3.4",
            "2001:db8::1234:5678:5.6.7.8",
            "2001:db8:3:4:5::192.0.2.33",
        ];
        let query = IPV6_ADDRESS.compile().unwrap();
        for ip in ips {
            assert!(query.is_match(ip));
        }
    }

    #[test]
    fn url() {
        let urls = [
            "http://foo.com/blah_blah",
            "http://foo.com/blah_blah/",
            "http://foo.com/blah_blah_(wikipedia)",
            "http://www.example.com/wpstyle/?p=364",
            "https://www.example.com/foo/?bar=baz&inga=42&quux",
            "http://userid:password@example.com:8080",
            "http://foo.com/blah_(wikipedia)#cite-1",
            "google.com",
            "www.google.com",
        ];
        let query = SIMPLE_URL.compile().unwrap();
        for url in urls {
            assert!(query.is_match(url), "Failed to match: {}", url);
        }
    }

    const EMAIL_CASES: &[&str] = &[
        "email@example.com",
        "firstname.lastname@example.com",
        "email@subdomain.example.com",
        "firstname+lastname@example.com",
        "1234567890@example.com",
        "email@example-one.com",
        "_______@example.com",
        "email@example.name",
        "email@example.museum",
        "email@example.co.jp",
        "firstname-lastname@example.com",
        "_@baz.com",
    ];

    #[test]
    fn simple_email() {
        let query = SIMPLE_EMAIL.compile().unwrap();
        for mail in EMAIL_CASES {
            assert!(query.is_match(mail), "Failed to match: {}", mail);
        }
    }

    #[test]
    fn email() {
        let query = EMAIL.compile().unwrap();
        for mail in EMAIL_CASES {
            assert!(query.is_match(mail), "Failed to match: {}", mail);
        }
    }
}
