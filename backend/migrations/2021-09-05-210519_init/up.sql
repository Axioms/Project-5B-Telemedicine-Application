-- Your SQL goes here
CREATE TABLE users ( 
    uuid                    CHAR(36)         NOT NULL PRIMARY KEY, 
    username                varchar(256)    NOT NULL UNIQUE,
    
    password_hint           TEXT,
    password_hash           BLOB            NOT NULL,
    salt                    BLOB            NOT NULL,
    
    password_iterations     INT UNSIGNED    NOT NULL,
    kdf_type                INT UNSIGNED    NOT NULL,
    kdf_iterations          INT UNSIGNED    NOT NULL,
    
    created_at              DATETIME        NOT NULL,
    updated_at              DATETIME        NOT NULL,
    totp_enable             BOOLEAN         NOT NULL DEFAULT FALSE,
    
    akey                    TEXT            NOT NULL,
    private_key             TEXT            NOT NULL,
    public_key              TEXT            NOT NULL,
    security_stamp          TEXT            NOT NULL,

    scope                   TEXT            NOT NULL
);