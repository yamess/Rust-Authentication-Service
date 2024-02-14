-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP CONSTRAINT IF EXISTS users_email_unique;

