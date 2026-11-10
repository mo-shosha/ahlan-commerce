variable "database_url" {
  type    = string
  default = getenv("DATABASE_URL")
}

locals {
  url = urlqueryset(urlqueryset(var.database_url, "sslmode", "disable"), "search_path", "public")

  dev_url = urlsetpath(local.url, "ahlan_commerce_atlas_dev")
}

env "local" {
  src = "file://db/schema/products.sql"
  url = local.url
  dev = local.dev_url

  migration {
    dir = "file://db/migrations"
  }
}
