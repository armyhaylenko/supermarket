$(async function() {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_products", {
        method: "GET",
        headers: {
            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
        }
    }).then(r => r.json());
    let parsed = JSON.parse(all_categories_json);

    // function contains(thisSet, thatSet) {
    //     if (thisSet === thatSet) {
    //         return true;
    //     } else {
    //         let flag = true;
    //         for (el in thatSet) {
    //             if (!thisSet.contains(el)) {
    //                 flag = false;
    //             } else continue;
    //         }
    //         return flag;
    //     }
    // }
    $("#jsGrid").jsGrid({
        width: "100%",
        height: "400px",
        sorting: true,
        paging: true,
        editing: false,
        data: parsed,
        fields: [
            { name: "product_id", title: "ID", type: "number" },
            { name: "product_name", title: "Name", type: "text" },
            { name: "descr", title: "Description", type: "text" },
            { name: "category_id", title: "Category ID", type: "number" }
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
                    let all_categories_json = await fetch("http://localhost:3000/api/utils/get_all_products", {
                        method: "GET",
                        headers: {
                            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
                        }
                    }).then(r => r.json());
                    let parsed = JSON.parse(all_categories_json);
                    console.log(parsed);
                    let result;
                    console.log(filter);
                    if (filter.category_id != undefined) {
                        result = parsed.filter(el => el.category_id == filter.category_id);
                    }
                    if (filter.product_id != undefined) {
                        result = parsed.filter(el => el.product_id == filter.product_id);
                    }
                    if (filter.product_name != "") {
                        result = parsed.filter(el => el.product_name == filter.product_name);
                    }
                    if (filter.descr != "") {
                        result = parsed.filter(el => el.descr == filter.descr);
                        console.log(result);
                    }
                    // if (filter.category_id == undefined) {
                    //     delete filter.category_id;
                    //     console.log("1");
                    // }
                    // if (filter.product_id == undefined) {
                    //     delete filter.product_id;
                    //     console.log("1");
                    // }
                    // if (filter.product_name == "") {
                    //     delete filter.product_name;
                    //     console.log("1");
                    // }
                    // if (filter.descr == "") {
                    //     delete filter.descr;
                    //     console.log("1");
                    // }
                    // filter = JSON.stringify(filter);
                    // console.log("before " + filter);
                    // result = parsed.filter(el => el.contains(filter));
                    // console.log("after " + filter);
                    // console.log("result " + result);
                    return result;
                }
            },

            fields: [
                { name: "product_id", title: "ID", type: "number" },
                { name: "product_name", title: "Name", type: "text" },
                { name: "descr", title: "Description", type: "text" },
                { name: "category_id", title: "Category ID", type: "number" }
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