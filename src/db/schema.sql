-- Tabla para almacenar los productos
CREATE TABLE products (
                          id_product SERIAL PRIMARY KEY,
                          name VARCHAR(255) NOT NULL,
                          price DECIMAL(10, 2) NOT NULL  -- Usamos DECIMAL para precios con dos decimales
);

-- Tabla para almacenar los usuarios
CREATE TABLE users (
                       id_user SERIAL PRIMARY KEY,
                       name VARCHAR(255) NOT NULL,
                       phone VARCHAR(20) NOT NULL,
                       address TEXT NOT NULL
);

-- Enum para el estado de la orden
CREATE TYPE order_status AS ENUM (
    'pending', 'preparing', 'on_the_way', 'delivered', 'cancelled'
);

-- Tabla para almacenar los repartidores
CREATE TABLE delivery_men (
                              id_delivery_man SERIAL PRIMARY KEY,
                              name VARCHAR(255) NOT NULL,
                              phone VARCHAR(20) NOT NULL
);


-- Tabla para almacenar las órdenes
CREATE TABLE orders (
                        id_order SERIAL PRIMARY KEY,
                        id_user INT NOT NULL,
                        id_delivery_man INT NOT NULL,
                        time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        state order_status NOT NULL DEFAULT 'pending',  -- Usamos el tipo ENUM para el estado
                        delivery_address TEXT NOT NULL,  -- Dirección de entrega
                        FOREIGN KEY (id_user) REFERENCES users(id_user) ON DELETE CASCADE,
                        FOREIGN KEY (id_delivery_man) REFERENCES delivery_men(id_delivery_man) ON DELETE CASCADE
);

-- Tabla para almacenar los detalles de la orden (relación muchos a muchos entre productos y órdenes)
CREATE TABLE order_details (
                               id_detail SERIAL PRIMARY KEY,
                               id_order INT NOT NULL,
                               id_product INT NOT NULL,
                               amount INT NOT NULL,  -- Cantidad de productos
                               FOREIGN KEY (id_order) REFERENCES orders(id_order) ON DELETE CASCADE,
                               FOREIGN KEY (id_product) REFERENCES products(id_product) ON DELETE CASCADE
);



-- Tabla para almacenar la localización del repartidor durante la entrega
CREATE TABLE delivery_locations (
                                    id_location SERIAL PRIMARY KEY,
                                    id_delivery_man INT NOT NULL,
                                    latitude FLOAT NOT NULL,  -- Latitud
                                    longitude FLOAT NOT NULL,  -- Longitud
                                    time_delivery_man TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                    FOREIGN KEY (id_delivery_man) REFERENCES delivery_men(id_delivery_man) ON DELETE CASCADE
);
