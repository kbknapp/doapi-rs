#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt:expr) => (println!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}
#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

// Taken from https://github.com/kbknapp/clap-rs
macro_rules! doapi_enum {
    (enum $e:ident { $($v:ident),+ } ) => {
        enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }

        impl $e {
            /// Lists all variants in a `Vec` of string slices
            fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (pub enum $e:ident { $($v:ident),+ } ) => {
        pub enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }

        impl $e {
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (#[derive($($d:ident),+)] enum $e:ident { $($v:ident),+ } ) => {
        #[derive($($d,)+)]
        enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }

        impl $e {
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (#[derive($($d:ident),+)] pub enum $e:ident { $($v:ident),+ } ) => {
        #[derive($($d,)+)]
        pub enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }

        impl $e {
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
}

macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);
