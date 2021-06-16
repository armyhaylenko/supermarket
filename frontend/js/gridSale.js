$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_sales", {
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
            { name: "receipt_id", title: "ID", type: "number" },
            { name: "product_upc", title: "Product UPC", type: "number" },
            { name: "qty", title: "Amount", type: "number" },
            { name: "price", title: "Price", type: "text" }
        ]
    });
    $("#jsGrid").jsGrid("cancelEdit");
    $("#jsGrid").jsGrid({
        onItemDeleting: function(args) {
            args.cancel = true;
        }
    });

    let filteredData = parsed;

    $("#filter").on('click', function() {
        $("#jsGrid").jsGrid({
            width: "100%",
            height: "600px",
            sorting: true,
            paging: true,
            editing: false,
            filtering: true,
            controller: {
                loadData: filter => loadDataController(filter, "get_all_sales", filteredData)
            },

            fields: [
                { name: "receipt_id", title: "ID", type: "number" },
                { name: "product_upc", title: "Product UPC", type: "number" },
                { name: "qty", title: "Amount", type: "number" },
                { name: "price", title: "Price", type: "text" }
            ]
        });
        $("#jsGrid").jsGrid("cancelEdit");
        $("#jsGrid").jsGrid({
            onItemDeleting: function(args) {
                args.cancel = true;
            }
        });
    })

    $("#print").on("click", function() {
        convertToCSVAndDownload(filteredData, "sales_");
    })

})