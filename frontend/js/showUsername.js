$(function () {
    let jwt = Cookies.get("ZLAGODA_AUTH_TOKEN");
    if(jwt !== null) {
        $("#login").remove();
        let decodedToken = JSON.parse(atob(jwt.split('.')[1]));
        let username = decodedToken.username;
        let user_role = decodedToken.user_role;
        $("#username").append(`<p>Username: ${username}</p> <p>User role: ${user_role}</p>`);
    }
})