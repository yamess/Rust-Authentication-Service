-- Your SQL goes here
ALTER TABLE "users" DROP COLUMN "updated_at";
ALTER TABLE "users" ADD COLUMN "updated_at" TIMESTAMP;

