use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, values) = delimited(
            tag("{"),
            separated_list1(tag(","), Self::parse_value),
            tag("}"),
        )(i)?;

        Ok((
            i,
            Self {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            },
        ))
    }

    fn parse_value(i: &str) -> IResult<&str, usize> {
        map(tuple((alpha1, tag("="), parse_number)), |(_, _, value)| {
            value
        })(i)
    }

    pub fn get(&self, id: &str) -> usize {
        match id {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => unreachable!(),
        }
    }

    pub fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    GreaterThan(String, usize, String),
    LessThan(String, usize, String),
    Accept,
    Reject,
    Forward(String),
}

impl Rule {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            Self::parse_greater_than,
            Self::parse_less_than,
            Self::parse_accept,
            Self::parse_reject,
            Self::parse_forward,
        ))(i)
    }

    fn parse_greater_than(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                alpha1::<&str, nom::error::Error<&str>>,
                tag(">"),
                terminated(parse_number, tag(":")),
                alpha1,
            )),
            |(id1, _, value, id2)| Self::GreaterThan(id1.to_string(), value, id2.to_string()),
        )(i)
    }

    fn parse_less_than(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                alpha1::<&str, nom::error::Error<&str>>,
                tag("<"),
                terminated(parse_number, tag(":")),
                alpha1,
            )),
            |(id1, _, value, id2)| Self::LessThan(id1.to_string(), value, id2.to_string()),
        )(i)
    }

    fn parse_accept(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("A")(i)?;
        Ok((i, Self::Accept))
    }

    fn parse_reject(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("R")(i)?;
        Ok((i, Self::Reject))
    }

    fn parse_forward(i: &str) -> IResult<&str, Self> {
        let (i, id) = alpha1(i)?;

        if id == "A" || id == "R" {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            )));
        }

        Ok((i, Self::Forward(id.to_string())))
    }
}

#[derive(Debug, PartialEq)]
pub struct Workflow {
    pub id: String,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, id) = alpha1(i)?;
        let (i, rules) = delimited(tag("{"), separated_list1(tag(","), Rule::parse), tag("}"))(i)?;

        Ok((
            i,
            Self {
                id: id.to_string(),
                rules,
            },
        ))
    }
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(i)
}
