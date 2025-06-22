-- Eliminar FK y borrar tabla
ALTER TABLE "order" DROP CONSTRAINT order_id_delivery_man_fkey;
DROP TABLE delivery_man;

-- Crear el tipo ENUM para rol
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('admin', 'delivery', 'customer');
    END IF;
END$$;

-- Agregar columnas
ALTER TABLE "user"
ADD COLUMN role user_role;

ALTER TABLE "user"
ADD COLUMN latitude VARCHAR,
ADD COLUMN longitude VARCHAR;