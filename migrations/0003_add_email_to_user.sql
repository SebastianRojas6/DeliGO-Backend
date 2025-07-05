-- Add "email" column to "user" table
ALTER TABLE "user"
ADD COLUMN email VARCHAR UNIQUE NOT NULL;