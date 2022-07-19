use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Operator {
  Plus,
  Minus,
  Star,
  Slash,
  Mod,
  LessThan,
  GreaterThan,
  Amp,
  AmpAmp,
  Pipe,
  PipePipe
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      match self {
        Operator::Plus => write!(f, "+"),
        Operator::Minus => write!(f, "-"),
        Operator::Star => write!(f, "*"),
        Operator::Slash => write!(f, "/"),
        Operator::Mod => write!(f, "%"),
        Operator::LessThan => write!(f, "<"),
        Operator::GreaterThan => write!(f, ">"),
        Operator::Amp => write!(f, "&"),
        Operator::AmpAmp => write!(f, "&&"),
        Operator::Pipe => write!(f, "|"),
        Operator::PipePipe => write!(f, "||"),
      }
    }
}

impl Operator {
  pub fn from(op: &'static str) -> Option<Operator> {
    match op {
      "+" => Some(Operator::Plus),
      "-" => Some(Operator::Minus),
      "*" => Some(Operator::Star),
      "/" => Some(Operator::Slash),
      "%" => Some(Operator::Mod),
      "<" => Some(Operator::LessThan),
      ">" => Some(Operator::GreaterThan),
      "&" => Some(Operator::Amp),
      "&&" => Some(Operator::AmpAmp),
      "|" => Some(Operator::Pipe),
      "||" => Some(Operator::PipePipe),
      _ => None
    }
  }

  pub fn to_str(&self) -> &'static str {
    match self {
      Operator::Plus => "+",
      Operator::Minus => "-",
      Operator::Star => "*",
      Operator::Slash => "/",
      Operator::Mod => "%",
      Operator::LessThan => "<",
      Operator::GreaterThan => ">",
      Operator::Amp => "&",
      Operator::AmpAmp => "&&",
      Operator::Pipe => "|",
      Operator::PipePipe => "||",
    }
  }
}
