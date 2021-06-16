$(function() {
    let response;
    $("#registr").on("click", async function() {
        let url = "http://localhost:3000/api/owned_product?action=create";
        let urlencoded = $("#form").serialize();
        let parsed = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        parsed.product_id = parseInt(parsed.product_id);
        parsed.units_in_stock = parseInt(parsed.units_in_stock);
        parsed.on_sale_product_upc = parsed.product_upc;
        parsed.is_on_sale = !!parsed.is_on_sale;
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
            alert("Owned product was successfully added to DB");
        } else {
            alert("Something went wrong with input data, check input fields and try again")
        }
    });
})