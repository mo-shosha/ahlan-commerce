--! list_products : (description?, published_at?)
select id, title, handle, description, price_cents, inventory_quantity,
       published, published_at, created_at, updated_at
from products
order by created_at asc, id asc;
