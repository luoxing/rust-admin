pub struct Error<> {
    code: u32,
    msg: &'static str,
}

pub const ERR_OK: Error =  Error{code: 0, msg: "success"};
