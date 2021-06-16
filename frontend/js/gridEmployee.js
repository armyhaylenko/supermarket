$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_employee", {
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
            { name: "empl_id", title: "ID", type: "number" },
            { name: "first_name", title: "First name", type: "text" },
            { name: "last_name", title: "Last name", type: "text" },
            { name: "patronymic", title: "Patronymic", type: "text" },
            { name: "user_role", title: "Role", type: "text" },
            { name: "join_date", title: "Join", type: "text" },
            { name: "phone_num", title: "Phone number", type: "text" },
            { name: "addr_street", title: "Street", type: "text" },
            { name: "addr_city", title: "City", type: "text" },
            { name: "addr_postal", title: "Postal", type: "text" },
            { name: "salary", title: "Salary", type: "number" }
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
                loadData: filter => loadDataController(filter, "get_all_employee")
            },

            fields: [
                { name: "empl_id", title: "ID", type: "number" },
                { name: "first_name", title: "First name", type: "text" },
                { name: "last_name", title: "Last name", type: "text" },
                { name: "patronymic", title: "Patronymic", type: "text" },
                { name: "user_role", title: "Role", type: "text" },
                { name: "join_date", title: "Join", type: "text" },
                { name: "phone_num", title: "Phone number", type: "text" },
                { name: "addr_street", title: "Street", type: "text" },
                { name: "addr_city", title: "City", type: "text" },
                { name: "addr_postal", title: "Postal", type: "text" },
                { name: "salary", title: "Salary", type: "number" }
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