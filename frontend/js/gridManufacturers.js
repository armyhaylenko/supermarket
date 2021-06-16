$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_manufacturers", {
        method: "GET",
        headers: {
            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
        }
    }).then(r => r.json());
    let parsed = JSON.parse(all_categories_json);
    $("#jsGrid").jsGrid({
        width: "100%",
        height: "400px",
        sorting: true,
        paging: true,
        editing: false,
        data: parsed,
        fields: [
            { name: "manufacturer_id", title: "ID", type: "number" },
            { name: "contract_id", title: "Contract ID", type: "number" },
            { name: "manufacturer_name", title: "Name", type: "text" },
            { name: "contract_sign_date", title: "Sign date", type: "text" },
            { name: "contract_end_date", title: "End date", type: "text" },
            { name: "tel_num", title: "Phone number", type: "text" },
            { name: "addr_street", title: "Street", type: "text" },
            { name: "addr_city", title: "City", type: "text" },
            { name: "addr_postal", title: "Postal", type: "text" },
            { name: "country", title: "Country", type: "text" }
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
            height: "400px",
            sorting: true,
            paging: true,
            editing: false,
            filtering: true,
            controller: {
                loadData: async function(filter) {
                    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_manufacturers", {
                        method: "GET",
                        headers: {
                            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
                        }
                    }).then(r => r.json());
                    let parsed = JSON.parse(all_categories_json);
                    console.log(parsed);
                    let result;
                    console.log(filter);
                    return parsed;
                }
            },

            fields: [
                { name: "manufacturer_id", title: "ID", type: "number" },
                { name: "contract_id", title: "Contract ID", type: "number" },
                { name: "manufacturer_name", title: "Name", type: "text" },
                { name: "contract_sign_date", title: "Sign date", type: "text" },
                { name: "contract_end_date", title: "End date", type: "text" },
                { name: "tel_num", title: "Phone number", type: "text" },
                { name: "addr_street", title: "Street", type: "text" },
                { name: "addr_city", title: "City", type: "text" },
                { name: "addr_postal", title: "Postal", type: "text" },
                { name: "country", title: "Country", type: "text" }
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