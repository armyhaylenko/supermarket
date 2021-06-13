$(async function () {
        let all_categories_json = JSON.parse(await fetch("http://localhost:3000/api/utils/get_all_categories", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json()));

        console.log(JSON.stringify(all_categories_json));

        $("#jsGrid").jsGrid({
            width: "100%",
            height: "100%",

            filtering: true,
            editing: false,
            sorting: true,
            paging: true,

            data: all_categories_json,

            fields: [
                {name: "category_id", type: "number", width: 200},
                {name: "category_name", type: "text", width: 300},
                {type: "control"}
            ]
        });
})