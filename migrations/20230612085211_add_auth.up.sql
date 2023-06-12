-- This file should undo anything in `up.sql`

CREATE TABLE "auth_client"
(
    id         SERIAL            NOT NULL,
    uuid       UUID              NOT NULL DEFAULT uuid_generate_v4(),

    nane       character varying NOT NULL,
    "key"      character varying NOT NULL,
    secret     character varying NOT NULL,

    created_at TIMESTAMP         NOT NULL DEFAULT now(),
    updated_at TIMESTAMP         NOT NULL DEFAULT now(),

    CONSTRAINT "UQ-auth-client-uuid" UNIQUE (uuid),
    CONSTRAINT "PK-auth-client" PRIMARY KEY ("id")
);

CREATE TABLE "token"
(
    id         SERIAL            NOT NULL,
    uuid       UUID              NOT NULL DEFAULT uuid_generate_v4(),
    token      character varying NOT NULL,

    user_id         INT               NOT NULL,
    auth_client_id  INT               NOT NULL,

    created_at TIMESTAMP         NOT NULL DEFAULT now(),
    updated_at TIMESTAMP         NOT NULL DEFAULT now(),
    expires_at TIMESTAMP         NOT NULL DEFAULT now(),
    revoked_at TIMESTAMP         NOT NULL DEFAULT now(),

    CONSTRAINT "UQ-token-uuid" UNIQUE (uuid),
    CONSTRAINT "FK-user" FOREIGN KEY (user_id) REFERENCES "user" (id),
    CONSTRAINT "FK-auth_client_id" FOREIGN KEY (auth_client_id) REFERENCES "auth_client" (id),
    CONSTRAINT "PK-token" PRIMARY KEY ("id")
);

CREATE TABLE "token_history"
(
    id         SERIAL            NOT NULL,
    uuid       UUID              NOT NULL DEFAULT uuid_generate_v4(),
    token_id   INT,


    ip_address character varying,
    country    character varying,
    loc        jsonb,
    user_agent character varying NOT NULL,

    created_at TIMESTAMP         NOT NULL DEFAULT now(),
    updated_at TIMESTAMP         NOT NULL DEFAULT now(),

    CONSTRAINT "UQ-token-history-uuid" UNIQUE (uuid),
    CONSTRAINT "FK-token" FOREIGN KEY (token_id) REFERENCES "token" (id),
    CONSTRAINT "PK-token-history" PRIMARY KEY ("id")
);
