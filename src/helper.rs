use nom::types::CompleteStr;
use nom::*;


fn signed_to_val(s: (Option<CompleteStr>, CompleteStr)) -> Result<i32, std::num::ParseIntError> {
    match s.0 {
        Some(CompleteStr("-")) => s.1.parse::<i32>().map(|v| -v),
        _ => s.1.parse::<i32>(),
    }
}

named!(pub i32_val<CompleteStr, i32>,
       map_res!(pair!(opt!(alt!(tag!("-") | tag!("+"))),  digit), signed_to_val));

named!(pub u32_val<CompleteStr, u32>,
       map_res!(digit, |s: CompleteStr| { s.parse::<u32>()} )
       );

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helper_parse_int32() {
        assert_eq!(i32_val(CompleteStr("123")), Ok((CompleteStr(""), 123)));
        assert_eq!(i32_val(CompleteStr("+123")), Ok((CompleteStr(""), 123)));
        assert_eq!(i32_val(CompleteStr("-123")), Ok((CompleteStr(""), -123)));
    }

    #[test]
    fn helper_parse_uint32() {
        assert_eq!(u32_val(CompleteStr("123")), Ok((CompleteStr(""), 123)));
    }
}
