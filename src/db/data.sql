INSERT INTO cliente (nombre, telefono, direccion)
VALUES ('Juan Pérez', '987654321', 'Av. Siempre Viva 123');

INSERT INTO producto (nombre, precio)
VALUES ('Pizza Margarita', 25.90);

INSERT INTO repartidor (nombre, telefono)
VALUES ('Laura Gómez', '912345678');

INSERT INTO pedido (id_cliente, id_repartidor, fecha_hora, estado)
VALUES (1, 1, '2025-04-14 18:30:00', 'Entregado');

INSERT INTO detalle_pedido (id_pedido, id_producto, cantidad)
VALUES (1, 1, 2);

INSERT INTO ubicacion_repartidor (id_repartidor, latitud, longitud, "timestamp")
VALUES (1, -12.0464, -77.0428, '2025-04-14 18:15:00');
