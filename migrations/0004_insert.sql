INSERT INTO public.product (id_product, name, price) VALUES
  (1, 'Pizza Margarita', 25.00),
  (2, 'Hamburguesa DDLR', 18.50),
  (3, 'Ensalada César', 15.00),
  (4, 'Taco al Pastor', 12.00),
  (5, 'Bebida Gaseosa', 6.50);

--La id 5 es mi cuenta xd

-- Orden 1
INSERT INTO public.order (id_order, id_user, id_delivery_man, time, delivery_address, order_status)
VALUES (1, 5, 201, '2025-07-05 12:00:00', 'Av. Siempre Viva 123', 'Pending');

-- Orden 2
INSERT INTO public.order (id_order, id_user, id_delivery_man, time, delivery_address, order_status)
VALUES (2, 5, 202, '2025-07-05 12:30:00', 'Calle Falsa 456', 'Preparing');

-- Orden 3
INSERT INTO public.order (id_order, id_user, id_delivery_man, time, delivery_address, order_status)
VALUES (3, 5, 203, '2025-07-05 13:00:00', 'Jr. Las Rosas 789', 'OnTheWay');

-- Orden 4
INSERT INTO public.order (id_order, id_user, id_delivery_man, time, delivery_address, order_status)
VALUES (4, 5, 204, '2025-07-05 13:30:00', 'Av. Primavera 654', 'Delivered');

-- Orden 5
INSERT INTO public.order (id_order, id_user, id_delivery_man, time, delivery_address, order_status)
VALUES (5, 5, 205, '2025-07-05 14:00:00', 'Calle Sol 321', 'Cancelled');
