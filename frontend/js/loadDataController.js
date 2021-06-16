async function loadDataController(filter, endpoint) {
    let all_categories_json = await fetch("http://localhost:3000/api/utils/" + endpoint, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${Cookies.get("ZLAGODA_AUTH_TOKEN")}`
        }
    }).then(r => r.json());
    Object.keys(filter).forEach(key => (filter[key] === undefined) || (filter[key] === "") ? delete filter[key] : {});
    let filterKeys = Object.keys(filter);
    console.log(filter);
    let takeSameProperties = (obj, _filterKeys) => {
        let resultingObject = {};
        for (let key of _filterKeys) {
            if(obj.hasOwnProperty(key)) {
                resultingObject[key] = obj[key];
            }
        }
        return resultingObject
    }
    let parsed = JSON.parse(all_categories_json);
    let r = parsed.filter(el => _.isEqual(takeSameProperties(el, filterKeys), filter));
    console.log(r);
    return r;
}