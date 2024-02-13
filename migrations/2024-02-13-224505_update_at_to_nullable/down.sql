-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "updated_at";
ALTER TABLE "users" ADD COLUMN "updated_at" TIMESTAMP NOT NULL;

