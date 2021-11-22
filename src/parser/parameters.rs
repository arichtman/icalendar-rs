use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::space0,
    combinator::{eof, map, opt},
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

#[cfg(test)]
use nom::error::ErrorKind;

use super::utils::valid_key_sequence;

/// Zero-copy version of [`crate::properties::Parameter`]
#[derive(PartialEq, Debug, Clone)]
pub struct Parameter<'a> {
    pub key: &'a str,
    pub val: Option<&'a str>,
}

impl<'a> From<Parameter<'a>> for crate::properties::Parameter {
    fn from(parameter: Parameter<'_>) -> crate::properties::Parameter {
        crate::properties::Parameter::new(parameter.key, parameter.val.unwrap_or(""))
    }
}

#[test]
fn test_parameter() {
    assert_parser!(
        parameter,
        ";KEY=VALUE",
        Parameter {
            key: "KEY",
            val: Some("VALUE")
        }
    );

    assert_parser!(
        parameter,
        "; KEY=VALUE",
        Parameter {
            key: "KEY",
            val: Some("VALUE")
        }
    );
    assert_parser!(
        parameter,
        "; KEY=VAL UE",
        Parameter {
            key: "KEY",
            val: Some("VAL UE")
        }
    );
    assert_parser!(
        parameter,
        "; KEY=",
        Parameter {
            key: "KEY",
            val: Some("")
        }
    );
    assert_parser!(
        parameter,
        ";KEY=VAL-UE",
        Parameter {
            key: "KEY",
            val: Some("VAL-UE")
        }
    );
    assert_parser!(
        parameter,
        ";KEY",
        Parameter {
            key: "KEY",
            val: None,
        }
    );

    assert_parser!(
        parameter,
        ";email=rust@hoodie.de",
        Parameter {
            key: "email",
            val: Some("rust@hoodie.de")
        }
    );
}

#[test]
fn test_parameter_with_dash() {
    assert_parser!(
        parameter,
        ";X-HOODIE-KEY=VALUE",
        Parameter {
            key: "X-HOODIE-KEY",
            val: Some("VALUE")
        }
    );
}

fn parameter<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Parameter<'a>, E> {
    map(
        tuple((
            preceded(
                tuple((tag(";"), space0)),
                valid_key_sequence, //key
            ),
            opt(preceded(
                tag("="),
                alt((eof, take_till1(|x| x == ';' || x == ':'))),
            )),
        )),
        |(key, val)| Parameter { key, val },
    )(input)
}

// parameter list
#[test]
pub fn parse_parameter_list() {
    assert_parser!(
        parameters,
        ";KEY=VALUE",
        vec![Parameter {
            key: "KEY",
            val: Some("VALUE")
        }]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=TODAY",
        vec![
            Parameter {
                key: "KEY",
                val: Some("VALUE")
            },
            Parameter {
                key: "DATE",
                val: Some("TODAY")
            }
        ]
    );

    assert_parser!(
        parameters,
        ";KEY=VALUE;DATE=20170218",
        vec![
            Parameter {
                key: "KEY",
                val: Some("VALUE")
            },
            Parameter {
                key: "DATE",
                val: Some("20170218")
            }
        ]
    );
}

pub fn parameters<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Parameter>, E> {
    many0(parameter)(input)
}