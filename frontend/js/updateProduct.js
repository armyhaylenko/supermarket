$(function() {
    let parsed;
    let selectCategory = $("#selectCategory");
    let selectUPC = $("#selectUPC")
    selectCategory.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_categories", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            selectCategory.append(`<option value="${parsed[i].category_id}">${parsed[i].category_name}</option>`)
        }

    });
    selectUPC.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_products", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            selectUPC.append(`<option value="${parsed[i].product_id}">${parsed[i].product_name}</option>`)
        }

    });
    let response;
    $("#registr").on("click", async function() {
        let url = "http://localhost:3000/api/product?action=update";
        let urlencoded = $("#form").serialize();
        let request = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        request.category_id = parseInt(request.category_id);
        request.product_id = parseInt(request.product_id);
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