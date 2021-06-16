$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_waybill", {
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
            { name: "waybill_id", title: "ID", type: "number" },
            { name: "waybill_date", title: "Date", type: "text" },
            { name: "base_price", title: "Base price", type: "text" },
            { name: "waybill_sum", title: "Sum", type: "text" },
            { name: "qty", title: "Amount", type: "nimber" },
            { name: "product_upc", title: "UPC", type: "text" },
            { name: "manufacturer_id", title: "manufacturer ID", type: "number" },
            { name: "empl_id", title: "Employee ID", type: "number" }
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
                    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_waybill", {
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
                { name: "waybill_id", title: "ID", type: "number" },
                { name: "waybill_date", title: "Date", type: "text" },
                { name: "base_price", title: "Base price", type: "text" },
                { name: "waybill_sum", title: "Sum", type: "text" },
                { name: "qty", title: "Amount", type: "nimber" },
                { name: "product_upc", title: "UPC", type: "text" },
                { name: "manufacturer_id", title: "manufacturer ID", type: "number" },
                { name: "empl_id", title: "Employee ID", type: "number" }
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