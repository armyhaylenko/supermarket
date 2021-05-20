use supermarket_management_system;

#[test]
fn app_req_claim_deserializarion() {
    std::env::set_var("JWT_DECODING_KEY", "c3VwZXItc2VjcmV0LWtleQo=");

    let x = async_pobaram::models::jwt::AppRequest {
        action: String::from("get_manufacturer"),
        token: String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k")
    };

    println!("{:?}", x.decode_token());
}
