$(function() {
    let parsed;
    let select = $("#selectManufacturer");
    select.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_manufacturers", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            select.append(`<option value="${parsed[i].manufacturer_id}">${parsed[i].manufacturer_name}</option>`)
        }

    });
    let response;
    $("#registr").on("click", async function() {
        let url = "http://localhost:3000/api/manufacturer?action=delete";
        let urlencoded = $("#form").serialize();
        let id = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        id.manufacturer_id = parseInt(id.manufacturer_id);
        let request = parsed.find(el => el.manufacturer_id === id.manufacturer_id);
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