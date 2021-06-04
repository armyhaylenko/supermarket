$(function() {
    $("#registr").on('click', postUser);

    function postUser() {
        // console.log($("#form").serialize());
        $.post("http://localhost:3000/login", $("#form").serialize());
    }
})