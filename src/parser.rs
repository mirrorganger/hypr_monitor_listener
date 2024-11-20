use serde::Deserialize;

pub fn parse_type<'a, T>(json_str: &'a str) -> Vec<T>
where
    T: Deserialize<'a>,
{
    let monitors: Vec<T> = match serde_json::from_str(json_str) {
        Ok(result) => result,
        Err(..) => vec![],
    };
    monitors
}
