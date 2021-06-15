$(function() {
    let parsed;
    let select = $("#selectAgree");
    select.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_return_agreement", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            select.append(`<option value="${parsed[i].return_agreement_id}">${parsed[i].return_agreement_id}</option>`)
        }

    });
    let response;
    $("#registr").on("click", async function() {
        let url = "http://localhost:3000/api/return_agreement?action=delete";
        let urlencoded = $("#form").serialize();
        let id = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        id.return_agreement_id = parseInt(id.return_agreement_id);
        let request = parsed.find(el => el.return_agreement_id === id.return_agreement_id);
        console.log(request);
        let token = Cookies.get("ZLAGODA_AUTH_TOKEN");
        await fetch(url, {
            method: "POST",
            mode: "cors",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify(request)
        }).then(function(resp) {
            response = resp.status;
            console.log(response);
        });
        if (response == 200) {
            alert("Success!");
        } else {
            alert("Something went wrong with input data, check input fields and try again")
        }
    });
})