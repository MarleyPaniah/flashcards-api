-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL DEFAULT gen_random_uuid(),
	"username" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMP NOT NULL DEFAULT now(),
	"last_login_at" TIMESTAMP NOT NULL DEFAULT now(),
	"is_verified" BOOL NOT NULL DEFAULT false,
	"is_deleted" BOOL NOT NULL DEFAULT false,
	PRIMARY KEY("id", "username")
);

