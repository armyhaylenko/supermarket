$(function() {
    $("#login").on("click", login)

    async function login() {
        let jwt = await fetch("http://localhost:3000/login", {
            method: "POST",
            mode: 'cors',
            credentials: 'omit',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: $("#form").serialize()
        }).then(resp => resp.text());
        Cookies.set("ZLAGODA_AUTH_TOKEN", jwt, { expires: 1 });
        let decodedToken = JSON.parse(atob(jwt.split('.')[1]));
        console.log(decodedToken);
        if (decodedToken != null) {
            window.location.href = "index.html";
        }
    }
})