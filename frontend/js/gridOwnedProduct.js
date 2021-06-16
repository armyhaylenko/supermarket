$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_owned_product", {
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
            { name: "product_upc", title: "UPC", type: "number" },
            { name: "product_id", title: "Product ID", type: "number" },
            { name: "sale_price", title: "Price", type: "text" },
            { name: "is_on_sale", title: "Sale", type: "checkbox" },
            { name: "units_in_stok", title: "Amout", type: "text" }
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
            height: "400px",
            sorting: true,
            paging: true,
            editing: false,
            filtering: true,
            controller: {
                loadData: filter => loadDataController(filter, "get_all_owned_product", filteredData)
            },

            fields: [
                { name: "product_upc", title: "UPC", type: "number" },
                { name: "product_id", title: "Product ID", type: "number" },
                { name: "sale_price", title: "Price", type: "text" },
                { name: "is_on_sale", title: "Sale", type: "checkbox" },
                { name: "units_in_stok", title: "Amout", type: "text" }
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
        convertToCSVAndDownload(filteredData, "owned_products_");
    })

})