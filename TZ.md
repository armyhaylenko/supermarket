#[TZ motherfucker]

too swag to use swagger.

## Admin endpoints

* GET /healthcheck - check if backend is working properly
* POST /admin/create_user username=egor&email=egor@king.com&password=nazi1488&role=cashier

    Username: max 150 symbols, email: valid email, password: max 24 symbols, role: *'cashier'* or *'manager'*
  
    Content-Type: application/x-www-form-urlencoded
  
    Response: message whether user was added or not (string)
* GET /admin/user/{user_name} - return user from db
    
    Returns: user struct json (id, username,
    email, user_role, full_name, created_at, updated_at) on success, error message on failure
  

## Login
POST /login


## Api endpoints

### Data endpoints

* POST /api/{target_table}?action=[create, update, delete] -- this is a generic 
structure of a data endpoint. For all the requests a full / almost full model is needed.
  Actions: 
  * create
    
    Create needs an instance of a model in json format, where primary key
    field is **null**
    
    Example: POST /api/employee?action=create *{"empl_id":null,"first_name":"Mykola","last_name":"Drabyna","patronymic":"Ivanovych","user_role":"cashier","salary":"0","join_date":"2021-05-25T08:13:03","phone_num":"+380976136133","addr_city":"Kyiv","addr_street":"Polyarna, 13","addr_postal":"05046"}*
    
  * update
    
    Update needs a full instance of the model with non-null primary keys.
      
  * delete
    
    Delete needs a mock instance of the model, where only primary key
    is read from user input.
    
The targets and their corresponding models are:

employee - ShopEmployee

client_card - ClientCard

manufacturer - Manufacturer

product - Product

owned_product - OwnedProduct

category - Category

waybill - Waybill

return_agreement - ReturnAgreement

create_receipt - CreateReceipt (important) // no ?action param

delete_receipt - Receipt // no ?action param

update_sale - Sale // no ?action param

### SQL Endpoints

TBD.