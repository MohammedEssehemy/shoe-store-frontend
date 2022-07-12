use reqwasm::http::Request;
use anyhow::Result;
use crate::models::ProductList;

pub async fn list_product() -> Result<ProductList> {
    let products: ProductList = Request::get("/api/products")
        .send()
        .await
        .unwrap()
        .json()
        .await?;
    return Ok(products);
}