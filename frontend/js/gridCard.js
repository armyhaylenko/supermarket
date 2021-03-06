$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_client_cards", {
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
            { name: "card_id", title: "ID", type: "number" },
            { name: "first_name", title: "First name", type: "text" },
            { name: "last_name", title: "Last name", type: "text" },
            { name: "patronymic", title: "Patronymic", type: "text" },
            { name: "phone_num", title: "Phone number", type: "text" },
            { name: "addr_street", title: "Street", type: "text" },
            { name: "addr_city", title: "City", type: "text" },
            { name: "addr_postal", title: "Postal", type: "text" },
            { name: "discount_rate", title: "Discount rate", type: "text" }
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
                loadData: filter => loadDataController(filter, "get_all_client_cards", filteredData)
            },

            fields: [
                { name: "card_id", title: "ID", type: "number" },
                { name: "first_name", title: "First name", type: "text" },
                { name: "last_name", title: "Last name", type: "text" },
                { name: "patronymic", title: "Patronymic", type: "text" },
                { name: "phone_num", title: "Phone number", type: "text" },
                { name: "addr_street", title: "Street", type: "text" },
                { name: "addr_city", title: "City", type: "text" },
                { name: "addr_postal", title: "Postal", type: "text" },
                { name: "discount_rate", title: "Discount rate", type: "text" }
            ]
        });
        $("#jsGrid").jsGrid("cancelEdit");
        $("#jsGrid").jsGrid({
            onItemDeleting: function(args) {
                args.cancel = true;
            }
        });
    });

    $("#print").on("click", function() {
        convertToCSVAndDownload(filteredData, "client_cards_");
    })

})