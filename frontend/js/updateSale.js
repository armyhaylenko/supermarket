$(function() {
    let parsed;
    let selectReceipt = $("#selectReceipt");
    let selectUPC = $("#selectUPC");
    selectReceipt.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_receipt", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            selectReceipt.append(`<option value="${parsed[i].receipt_id}">${parsed[i].receipt_id}</option>`)
        }
    });
    selectUPC.one("focus", async function() {
        let url = "http://localhost:3000/api/utils/get_all_products_by_receipt";
        let urlencoded = selectReceipt.val();
        console.log(urlencoded);
        let postJSON = { receipt_id: parseInt(urlencoded) };
        console.log(JSON.stringify(postJSON));
        let token = Cookies.get("ZLAGODA_AUTH_TOKEN");
        let response = await fetch(url, {
            method: "POST",
            mode: "cors",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            },
            body: JSON.stringify(postJSON)
        }).then(resp => resp.json());
        let parsedResponse = JSON.parse(response);
        console.log(JSON.stringify(parsedResponse));
        let ids = parsedResponse.map(resp => resp.receipt_id);
        for (let i = 0; i < ids.length; i++) {
            selectUPC.append(`<option value="${parsedResponse[i].product_upc}">${parsedResponse[i].product_upc}</option>`);
        }
    });
    $("#registr").on('click', async function() {
        let url = "http://localhost:3000/api/update_sale";
        let urlencoded = $("#form").serialize();
        let parsed = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        parsed.price = parseInt(parsed.price);
        parsed.receipt_id = parseInt(parsed.receipt_id);
        parsed.qty = parseInt(parsed.qty);
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
            alert("Success!");
        } else {
            alert("Something went wrong with input data, check input fields and try again")
        }
    })
})