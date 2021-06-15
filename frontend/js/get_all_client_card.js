$(function() {
    let select = $("#selectClientCard");
    select.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_client_cards", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        let parsed = JSON.parse(all_products_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            select.append(`<option value="${parsed[i].card_id}">${parsed[i].card_id}</option>`)
        }

    })
})