$(function() {
    let i = 0;
    let saleArray = [];
    let currentInstance;
    let parsedClientCards;
    let selectClientCard = $("#selectClientCard");
    selectClientCard.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_client_cards", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsedClientCards = JSON.parse(all_products_json);
        let len = parsedClientCards.length;
        for (let i = 0; i < len; i++) {
            selectClientCard.append(`<option value="${parsedClientCards[i].card_id}">${parsedClientCards[i].card_id}</option>`)
        }

    });
    let parsedEmployee;
    let selectEmployee = $("#selectEmployee");
    selectEmployee.one("focus", async function() {
        let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_employee", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
            }
        }).then(r => r.json());
        parsedEmployee = JSON.parse(all_products_json);
        let len = parsedEmployee.length;
        for (let i = 0; i < len; i++) {
            selectEmployee.append(`<option value="${parsedEmployee[i].empl_id}">${parsedEmployee[i].first_name} ${parsedEmployee[i].last_name}</option>`)
        }

    });
    $("#add").on("click", addSale);

    function addSale() {
        $("#sales").append(`<form id="sale` + i + `">
        <div class="row px-3"> <label class="mb-1">
            <h6 class="mb-0 text-sm">Product upc</h6></label>
            <select class="mb-4 rounded border-0" required name="product_upc" id="selectUPC` + i + `">
            <option selected value="null">Select upc</option>
        </select>
        </div>
        <div class="row px-3"> <label class="mb-1">
            <h6 class="mb-0 text-sm">Amount</h6></label>
            <input class="mb-4 rounded border-0" required name="qty" type="number" placeholder="Enter amount" id="qty` + i + `">
        </div>
        <div class="row px-3"> <label class="mb-1">
            <h6 class="mb-0 text-sm">Price</h6></label>
            <input class="mb-4 rounded border-0" required name="price" type="text" placeholder="Enter price" id="price` + i + `">
        </div>
        </form>`);
        let parsedOwnedProduct;
        let selectOwnedProduct = $("#selectUPC" + i);
        selectOwnedProduct.one("focus", async function() {
            let all_products_json = await fetch("http://localhost:3000/api/utils/get_all_owned_product", {
                method: "GET",
                headers: {
                    Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
                }
            }).then(r => r.json());
            parsedOwnedProduct = JSON.parse(all_products_json);
            let len = parsedOwnedProduct.length;
            for (let i = 0; i < len; i++) {
                selectOwnedProduct.append(`<option value="${parsedOwnedProduct[i].product_upc}">${parsedOwnedProduct[i].product_upc}</option>`)
            }

        });
        let currentSale = $("#sale" + i).serialize();
        currentSale = Object.fromEntries(
            currentSale.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        console.log(currentSale);
        let qty = $("#qty" + i + "").val();
        console.log(qty);
        let price = $("#price" + i + "").val();
        currentSale.qty = parseInt(qty);
        currentSale.price = parseFloat(price);
        saleArray[i] = currentSale;
        console.log(currentSale);

        i++;
    }
    $("#registr").on("click", function() {
        let urlencoded = $("#form").serialize();
        console.log(urlencoded);
        urlencoded = Object.fromEntries(
            urlencoded.split('&')
            .map(s => s.split('='))
            .map(pair => pair.map(decodeURIComponent)));
        let createReceipt = {};
        createReceipt.receipt_date = urlencoded.receipt_date;
        createReceipt.client_card_id = parseInt(urlencoded.card_id);
        createReceipt.empl_id = parseInt(urlencoded.empl_id);
        createReceipt.sales = saleArray;
        console.log(JSON.stringify(createReceipt));

    });

})