-- Your SQL goes here
CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_status as enum ( 'waiting_mail_validation', 'waiting_phone_validation', 'validated' );

CREATE TABLE "user"
(
    id         SERIAL            NOT NULL,
    uuid       UUID              NOT NULL DEFAULT uuid_generate_v4(),
    status     user_status       NOT NULL DEFAULT 'waiting_mail_validation',
    firstname  character varying NOT NULL,
    lastname   character varying NOT NULL,
    password   character varying NOT NULL,
    phone      character varying NOT NULL,
    email      character varying NOT NULL,

    created_at TIMESTAMP         NOT NULL DEFAULT now(),
    updated_at TIMESTAMP         NOT NULL DEFAULT now(),

    CONSTRAINT "UQ-user-uuid" UNIQUE (uuid),
    CONSTRAINT "UQ-user-email" UNIQUE (email),
    CONSTRAINT "UQ-user-phone" UNIQUE (phone),
    CONSTRAINT "PK-user" PRIMARY KEY ("id")
);