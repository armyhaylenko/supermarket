$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_categories", {
        method: "GET",
        headers: {
            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
        }
    }).then(r => r.json());
    let parsed = JSON.parse(all_categories_json);
    $("#jsGrid").jsGrid({
        width: "100%",
        height: "600px",
        sorting: true,
        paging: true,
        editing: false,
        data: parsed,
        fields: [
            { name: "category_id", title: "ID", type: "number" },
            { name: "category_name", title: "Name", type: "text" }
        ]
    });
    $("#jsGrid").jsGrid("cancelEdit");
    $("#jsGrid").jsGrid({
        onItemDeleting: function(args) {
            args.cancel = true;
        }
    });


    $("#filter").on('click', function() {
        $("#jsGrid").jsGrid({
            width: "100%",
            height: "600px",
            sorting: true,
            paging: true,
            editing: false,
            filtering: true,
            controller: {
                loadData: async function(filter) {
                    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_categories", {
                        method: "GET",
                        headers: {
                            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
                        }
                    }).then(r => r.json());
                    let parsed = JSON.parse(all_categories_json);
                    console.log(filter);
                    let result;
                    if (filter.category_id !== undefined) {
                        result = [parsed.find(el => el.category_id == filter.category_id)];
                        console.log(result);
                    }
                    if (filter.category_name !== "") {
                        result = [parsed.find(el => el.category_name == filter.category_name)];
                        console.log(result);
                    }
                    console.log(result);
                    console.log(parsed);
                    return result;
                }
            },

            fields: [
                { name: "category_id", title: "ID", type: "number" },
                { name: "category_name", title: "Name", type: "text" }
            ]
        });
        $("#jsGrid").jsGrid("cancelEdit");
        $("#jsGrid").jsGrid({
            onItemDeleting: function(args) {
                args.cancel = true;
            }
        });
    })

})