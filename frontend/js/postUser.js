$(function() {
    $("#registr").on('click', postUser);

    function postUser() {
        // console.log($("#form").serialize());
        $.ajax({
            url: "http://localhost:3000/admin/create_user",
            headers: { Authorization: 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6IkpvaG4gRG9lIiwidXNlcl9yb2xlIjoibWFuYWdlciIsImV4cCI6MTcxNjE0Mzc3NH0.MJ430Tzz4GUyUQv3i3q_-dn92CsMSFvbqkE2_JYZ74k' },
            data: $("#form").serialize()
        });
    }
})