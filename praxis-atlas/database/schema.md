# Database Schema

## Common Types

- `id`: integer [primary key]
- `title`: varchar(100)
- `description`: text
- `created_at`: timestamp
- `updated_at`: timestamp

## Schema Overview

![Entity Relationship Diagram](../assets/erd.png)

## Schema Conventions

- All tables include timestamps (`created_at`, `updated_at`)
- Foreign keys use `<entity>_id` naming convention
- Soft deletes implemented where appropriate
- UUID primary keys for distributed-safe IDs
