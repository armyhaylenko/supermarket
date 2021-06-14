$(function() {
    $(".hover").addClass("disabled");
    let jwt = Cookies.get("ZLAGODA_AUTH_TOKEN");
    if (jwt !== undefined) {
        $("#login").remove();
        let decodedToken = JSON.parse(atob(jwt.split('.')[1]));
        let username = decodedToken.username;
        let user_role = decodedToken.user_role;
        console.log(user_role);
        if (user_role == "cashier") {
            $(".cashier").removeClass("disabled");
        } else {
            $(".hover").removeClass("disabled");
        }
        $("#username").append(` <p class="bg-c3 badge loginB m-3" id="login" title="Log out">Username: ${username}</p>
                                <p class="bg-c3 badge loginB m-3">Role: ${user_role}</p>`);
        $("#login").on('click', function() {
            let bool = confirm("Do you want to log out?");
            if (bool) {
                Cookies.remove("ZLAGODA_AUTH_TOKEN");
                location.reload();
            }
        });
    }
})