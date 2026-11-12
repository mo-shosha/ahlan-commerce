--! list_published_products : (description?, published_at?)
select id, title, handle, description, price_cents, inventory_quantity,
       published, published_at, created_at, updated_at
from products
where published = true
order by published_at desc nulls last, created_at asc, id asc;
