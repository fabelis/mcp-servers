# shopify Server

**Version:** 0.1.0  
**Total Tools:** 11

<details>
<summary><strong>CreateOrder</strong></summary>

**Description:** Create a new order in the Shopify store.

**Parameters:**
- `line_items`: List of products in the order
- `customer_id`: ID of the customer

</details>

<details>
<summary><strong>CreateProduct</strong></summary>

**Description:** Create a new product in the Shopify store.

**Parameters:**
- `title`: Title of the product
- `body_html`: Product description
- `vendor`: Product vendor
- `product_type`: Type of the product
- `price`: Price of the product
- `image_url`: URL of the product image

</details>

<details>
<summary><strong>DeleteOrder</strong></summary>

**Description:** Delete an order from the Shopify store.

**Parameters:**
- `order_id`: ID of the order to delete

</details>

<details>
<summary><strong>DeleteProduct</strong></summary>

**Description:** Delete a product from the Shopify store.

**Parameters:**
- `product_id`: ID of the product to delete

</details>

<details>
<summary><strong>GetOrder</strong></summary>

**Description:** Retrieve details of a specific order by its ID.

**Parameters:**
- `order_id`: ID of the order to retrieve

</details>

<details>
<summary><strong>GetProduct</strong></summary>

**Description:** Fetch details of a specific product.

**Parameters:**
- `product_id`: ID of the product to retrieve

</details>

<details>
<summary><strong>GetSalesData</strong></summary>

**Description:** Retrieve sales data from the Shopify store.

**Parameters:**
_None_

</details>

<details>
<summary><strong>ListCustomers</strong></summary>

**Description:** List all customers in the Shopify store.

**Parameters:**
_None_

</details>

<details>
<summary><strong>ListProducts</strong></summary>

**Description:** Retrieve all products in a Shopify store.

**Parameters:**
_None_

</details>

<details>
<summary><strong>UpdateProduct</strong></summary>

**Description:** Update an existing product in the Shopify store.

**Parameters:**
- `product_id`: ID of the product to update
- `title`: New title for the product
- `body_html`: New product description
- `vendor`: New product vendor
- `product_type`: New type of the product
- `price`: New price of the product

</details>

<details>
<summary><strong>AddProductMedia</strong></summary>

**Description:** Attach media (images, videos, or models) to an existing Shopify product using GraphQL.

**Parameters:**
- `product_id`: The Shopify product GID (e.g., gid://shopify/Product/1234567890).
- `image_url`: URL of the product image
- `image_alt`: Alt text for the image

</details> 