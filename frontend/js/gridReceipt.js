$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_receipt", {
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
            { name: "receipt_id", title: "ID", type: "number" },
            { name: "receipt_date", title: "Date", type: "text" },
            { name: "receipt_sum", title: "Sum", type: "text" },
            { name: "VAT", title: "VAT", type: "text" },
            { name: "client_card_id", title: "Client card ID", type: "number" },
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
                loadData: filter => loadDataController(filter, "get_all_receipt")
            },

            fields: [
                { name: "receipt_id", title: "ID", type: "number" },
                { name: "receipt_date", title: "Date", type: "text" },
                { name: "receipt_sum", title: "Sum", type: "text" },
                { name: "VAT", title: "VAT", type: "text" },
                { name: "client_card_id", title: "Client card ID", type: "number" },
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