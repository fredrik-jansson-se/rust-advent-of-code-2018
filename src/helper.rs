use nom::types::CompleteStr;
use nom::*;

named!(pub i32_val<CompleteStr, i32>,
       map_res!(digit, |s: CompleteStr| s.parse::<i32>()));

named!(pub u32_val<CompleteStr, u32>,
       map_res!(digit, |s: CompleteStr| { s.parse::<u32>()} )
       );

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helper_parse_int32() {
        assert_eq!(i32_val(CompleteStr("123")), Ok((CompleteStr(""), 123)));
        assert_eq!(i32_val(CompleteStr("-123")), Ok((CompleteStr(""), -123)));
    }

    #[test]
    fn helper_parse_uint32() {
        assert_eq!(u32_val(CompleteStr("123")), Ok((CompleteStr(""), 123)));
    }
}
