extern crate md5;


pub fn login(user_name: &str, pwd: &str) -> Result<bool, &'static str> {
	let user_pwd = get_user_pwd(&user_name);
	if user_pwd.is_empty() {
		return Err("用户不存在！");
	}
	let digest = format!("{:x}", md5::compute(pwd));
	if digest != user_pwd {
		return Err("用户密码不正确");
	}
	Ok(true)
}

fn get_user_pwd(user_name: &str) ->  &'static str {
    match user_name{
        "hg" => "f3c6ff774b1c1c3a839dbda37215f636", //pyyx123
        _ => "",
    }
}