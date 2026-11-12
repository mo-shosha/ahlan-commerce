--! update_product_publication (published_at?) : (description?, published_at?)
update products
set published = :published,
    published_at = :published_at,
    updated_at = :updated_at
where id = :id
returning id, title, handle, description, price_cents, inventory_quantity,
          published, published_at, created_at, updated_at;
