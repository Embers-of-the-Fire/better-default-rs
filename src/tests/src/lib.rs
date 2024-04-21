#![allow(dead_code)]

#[test]
fn test_derive_struct() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Struct {
        #[default(expr = "123".to_owned())]
        name: String,
        #[default = 10]
        id: usize,
    }

    assert_eq!(
        Struct {
            name: "123".to_owned(),
            id: 10
        },
        Default::default(),
    );
}

#[test]
fn test_derive_struct_override() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Struct {
        #[default(expr = "123".to_owned(), expr = "456".to_owned())]
        name: String,
        #[default = 10]
        id: usize,
    }

    assert_eq!(
        Struct {
            name: "456".to_owned(),
            id: 10
        },
        Default::default(),
    );
}

#[test]
fn test_derive_tuple_struct() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Struct(
        #[default(expr = "123".to_owned())] String,
        #[default = 10] usize,
    );

    assert_eq!(Struct("123".to_owned(), 10), Default::default(),);
}

#[test]
fn test_derive_unit_struct() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    struct Struct;

    assert_eq!(Struct, Default::default(),);
}

#[test]
fn test_derive_struct_const() {
    use fancy_default::ConstDefault;

    #[derive(Debug, ConstDefault, PartialEq, Eq)]
    struct Struct<'a> {
        #[default(expr = "123")]
        name: &'a str,
        #[default = 10]
        id: usize,
    }

    assert_eq!(
        Struct {
            name: "123",
            id: 10
        },
        Struct::DEFAULT,
    );
}

#[test]
fn test_derive_enum() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    enum Enum {
        #[default]
        Plain,
    }

    assert_eq!(Enum::Plain, Default::default());
}

#[test]
fn test_derive_enum_struct() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    enum Enum {
        Plain,
        #[default = true]
        Struct {
            #[default(expr = "123".to_owned())]
            name: String,
        },
    }

    assert_eq!(
        Enum::Struct {
            name: "123".to_owned()
        },
        Default::default()
    );
}

#[test]
fn test_derive_enum_tuple() {
    use fancy_default::derive::Default;

    #[derive(Debug, Default, PartialEq, Eq)]
    enum Enum {
        Plain,
        Struct {
            #[default(expr = "123".to_owned())]
            name: String,
        },
        #[default]
        Tuple(#[default = 10] usize),
    }

    assert_eq!(Enum::Tuple(10), Default::default());
}

#[test]
fn test_derive_enum_const() {
    use fancy_default::ConstDefault;

    #[derive(Debug, ConstDefault, PartialEq, Eq)]
    enum Enum {
        Plain,
        Struct {
            #[default(expr = "123".to_owned())]
            name: String,
        },
        #[default]
        Tuple(#[default = 10] usize),
    }

    assert_eq!(Enum::Tuple(10), Enum::DEFAULT);
}

#[test]
fn test_derive_variant() {
    use fancy_default::VariantDefault;

    #[derive(Debug, VariantDefault, PartialEq, Eq)]
    #[variant(const)]
    enum Enum {
        Plain,
        #[variant(const = false)]
        Struct {
            #[default(expr = "123".to_owned())]
            name: String,
        },
        #[default]
        Tuple(#[default = 10] usize),
    }

    assert_eq!(Enum::PLAIN, Enum::Plain);
    assert_eq!(
        Enum::default_struct(),
        Enum::Struct {
            name: "123".to_owned()
        }
    );
}
