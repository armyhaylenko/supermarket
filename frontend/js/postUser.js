$(function() {
    $("#registr").on('click', postUser);

    async function postUser() {

        let response = await fetch(
            "http://localhost:3000/admin/create_user", {
                method: 'POST',
                mode: 'cors',
                credentials: 'omit',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                    'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k'
                },
                body: $("#form").serialize()
            }

        ).then(resp => resp.text());
        console.log(response);
        if (response == "Success") {
            window.location.href = "login.html";
        }
    }
})