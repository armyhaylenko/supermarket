$(function() {
    let select = $("#selectCategory");
    select.one("focus", async function() {
        let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_categories", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        let parsed = JSON.parse(all_categories_json);
        let len = parsed.length;
        for (let i = 0; i < len; i++) {
            select.append(`<option value="${parsed[i].category_id}">${parsed[i].category_name}</option>`)
        }

    })
})