$(function() {
    let response;
    $("#registr").on("click", async function() {
        let url = "http://localhost:3000/api/return_agreement?action=create";
        let urlencoded = $("#form").serialize();
        let parsed = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        parsed.qty = parseInt(parsed.qty);
        parsed.empl_id = parseInt(parsed.empl_id);
        parsed.manufacturer_id = parseInt(parsed.manufacturer_id);
        parsed.return_agreement_sum = 0;
        let token = Cookies.get("ZLAGODA_AUTH_TOKEN");
        await fetch(url, {
            method: "POST",
            mode: "cors",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify(parsed)
        }).then(function(resp) {
            response = resp.status;
            console.log(response);
        });
        if (response == 200) {
            alert("Return agreement was successfully added to DB");
        } else {
            alert("Something went wrong with input data, check input fields and try again")
        }
    });
})