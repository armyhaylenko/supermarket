#[cfg(test)]
extern crate libc;

use std::ffi::CString;
use std::time::Duration;

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

// run this only when the app is used for test purposes! restart after the test completes due to response caching
#[test]
fn test_employee_creation_and_deletion() {
    let create_cmd: CString = CString::new(r#"curl -XPOST -H\
     'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k'\
     -H 'Content-Type: application/json'\
     http://localhost:3000/api/employee?action=create --data-binary '{"empl_id":null,"first_name":"Mykola","last_name":"Drabyna","patronymic":"Ivanovych","user_role":"cashier","salary":"0","join_date":"2021-05-25T08:13:03","phone_num":"+380976136133","addr_city":"Kyiv","addr_street":"Polyarna, 13","addr_postal":"05046"}'"#)
        .unwrap();
    let get_most_recent_employee_cmd: CString =
        CString::new(r#"curl http://localhost:3000/api/tests/get_most_recent_employee > empl_id"#).unwrap();
    let empl_id = unsafe {
        match_exit_code(create_cmd);
        std::thread::sleep(Duration::from_secs(5u64));
        match_exit_code(get_most_recent_employee_cmd);
        std::fs::read_to_string("./empl_id").unwrap()
    };
    let delete_cmd: CString = CString::new(format!(r#"curl -XPOST -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k'\
     -H 'Content-Type: application/json'\
     http://localhost:3000/api/employee?action=delete --data-binary '{{"empl_id": {},"first_name":"Mykola","last_name":"Drabyna","patronymic":"Ivanovych","user_role":"cashier","salary":"0","join_date":"2021-05-25T08:13:03","phone_num":"+380976136133","addr_city":"Kyiv","addr_street":"Polyarna, 13","addr_postal":"05046"}}'"#, empl_id)).unwrap();

    unsafe { match_exit_code(delete_cmd) };
}
