$(function() {
    $("#registr").on("click", validate);

    function validate() {
        let pass = $("#password").val();
        let conf = $("#conf").val();
        if (pass != conf) {
            let error = `<p class="text-danger">*Passwords are not equal</p>`
            $("#error").append(error);
            event.preventDefault();
        }

    }
    $(".password").validate({
        rules: {
            password: {
                minlength: 5
            },
            password_confirm: {
                minlength: 5,
                equalTo: "#password"
            }
        }
    })
})