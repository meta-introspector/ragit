use super::SchemaType;
use serde_json::Value;

pub struct JsonishLiteral<'a> {
    pub s: &'a str,
    pub integers: Vec<(usize, usize)>,
    pub floats: Vec<(usize, usize)>,
    pub braces: Vec<(usize, usize)>,
    pub brackets: Vec<(usize, usize)>,
    pub likely_to_be_broken_json: bool,
}

impl<'a, 'b> JsonishLiteral<'a> {
    pub fn get_matches(&'a mut self, r#type: &'b SchemaType) -> super::JsonMatch<'a> {
        match r#type {
            SchemaType::Integer => match self.integers.len() {
                0 => super::JsonMatch::NoMatch,
                1 => super::JsonMatch::Match(self.s.get(self.integers[0].0..self.integers[0].1).unwrap()),
                _ => {
                    let mut parsed_integers = vec![];
                    let mut selected_str = self.s;

                    for (start, end) in self.integers.iter() {
                        let s = self.s.get(*start..*end).unwrap();
                        let Ok(n) = s.parse::<i128>() else {
                            continue;
                        };

                        // If the LLM outputs the same literal multiple times, that's fine.
                        match parsed_integers.last() {
                            None => {
                                parsed_integers.push(n);
                                selected_str = s;
                            }
                            Some(l) if *l != n => {
                                return super::JsonMatch::MultipleMatches;
                            }
                            Some(_) => {}
                        }
                    }

                    if parsed_integers.is_empty() {
                        super::JsonMatch::NoMatch
                    } else {
                        super::JsonMatch::Match(selected_str)
                    }
                }
            },
            SchemaType::Float => match self.floats.len() {
                0 => super::JsonMatch::NoMatch,
                1 => super::JsonMatch::Match(self.s.get(self.floats[0].0..self.floats[0].1).unwrap()),
                _ => {
                    let mut parsed_floats = vec![];
                    let mut selected_str = self.s;

                    for (start, end) in self.floats.iter() {
                        let s = self.s.get(*start..*end).unwrap();
                        let Ok(n) = s.parse::<f64>() else {
                            continue;
                        };

                        // If the LLM outputs the same literal multiple times, that's fine.
                        match parsed_floats.last() {
                            None => {
                                parsed_floats.push(n);
                                selected_str = s;
                            }
                            Some(l) if *l != n => {
                                return super::JsonMatch::MultipleMatches;
                            }
                            Some(_) => {}
                        }
                    }

                    if parsed_floats.is_empty() {
                        super::JsonMatch::NoMatch
                    } else {
                        super::JsonMatch::Match(selected_str)
                    }
                }
            },
            ty @ (SchemaType::Array(_) | SchemaType::Object(_)) => {
                let l = if let SchemaType::Array(_) = ty {
                    &self.brackets
                } else {
                    &self.braces
                };

                match l.len() {
                    0 => super::JsonMatch::NoMatch,
                    1 => super::JsonMatch::Match(self.s.get(l[0].0..l[0].1).unwrap()),
                    _ => {
                        let mut parsed_jsons = vec![];
                        let mut selected_str = self.s;

                        for (start, end) in l.iter() {
                            let s = self.s.get(*start..*end).unwrap();
                            let Ok(n) = serde_json::from_str::<Value>(s) else {
                                self.likely_to_be_broken_json = true;
                                continue;
                            };

                            // If the LLM outputs the same literal multiple times, that's fine.
                            match parsed_jsons.last() {
                                None => {
                                    parsed_jsons.push(n);
                                    selected_str = s;
                                }
                                Some(l) if *l != n => {
                                    return super::JsonMatch::MultipleMatches;
                                }
                                Some(_) => {}
                            }
                        }

                        if parsed_jsons.is_empty() {
                            super::JsonMatch::NoMatch
                        } else {
                            super::JsonMatch::Match(selected_str)
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
