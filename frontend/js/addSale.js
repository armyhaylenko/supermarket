$("#add").on("click", addSale);
let i = 0;

function addSale() {
    // $("[name*='sale" + i + "']").submit();
    // i++;
    $("#sales").append(`<form method="POST" name="sale` + i + `">
    <div class="row px-3"> <label class="mb-1">
        <h6 class="mb-0 text-sm">Product upc</h6></label>
        <select class="mb-4 rounded border-0" required name="product_upc">
        <option selected value="null">Select upc</option>
        <option value="product_upc1">product_upc1</option>
    </select>
    </div>
    <div class="row px-3"> <label class="mb-1">
        <h6 class="mb-0 text-sm">Amount</h6></label>
        <input class="mb-4 rounded border-0" required name="qty" type="number" placeholder="Enter amount">
    </div>
    </form>`)
}