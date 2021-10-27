-- Your SQL goes here
CREATE TABLE users ( 
    uuid                    CHAR(36)         NOT NULL PRIMARY KEY, 
    created_at              DATETIME        NOT NULL,
    updated_at              DATETIME        NOT NULL,

    username                varchar(256)    NOT NULL UNIQUE,
    
    password_hash           BLOB            NOT NULL,
    salt                    BLOB            NOT NULL,
    password_hint           TEXT,
    
    akey                    TEXT            NOT NULL,
    private_key             TEXT            NOT NULL,
    public_key              TEXT            NOT NULL,

    security_stamp          TEXT            NOT NULL,
    totp_enable             BOOLEAN         NOT NULL DEFAULT FALSE,

    password_iterations     INT UNSIGNED    NOT NULL,
    kdf_type                INT UNSIGNED    NOT NULL,
    kdf_iterations          INT UNSIGNED    NOT NULL,
    scope                   TEXT            NOT NULL
);