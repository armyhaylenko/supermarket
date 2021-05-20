#[cfg(test)]
extern crate libc;

use std::ffi::CString;

unsafe fn match_exit_code(cmd: CString) {
    let exit_code = libc::system(cmd.as_ptr());
    match exit_code {
        0 => (),
        _ => panic!("Non-zero exit code returned from curl: {}", exit_code),
    }
}

#[test]
fn test_user_creation() {
    let cmd: CString =
            CString::new(r#"curl -XPOST -H\
            'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k'\
             http://localhost:3000/admin/create_user -H 'Content-Type: application/x-www-form-urlencoded;charset=UTF-8'\
             --data-binary 'username=alex&email=bostonaqua@gmail.com&password=iwantaps5&user_role=manager'"#).unwrap();
    unsafe { match_exit_code(cmd) }
}

#[test]
fn test_user_retrieval() {
    let cmd: CString = CString::new(r#"curl http://localhost:3000/admin/user/alex"#).unwrap();

    unsafe { match_exit_code(cmd) }
}
