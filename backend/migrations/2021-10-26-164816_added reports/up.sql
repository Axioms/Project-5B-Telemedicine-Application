-- Your SQL goes here
CREATE TABLE reports ( 
    uuid                    CHAR(36)         NOT NULL PRIMARY KEY, 
    user_uuid               CHAR(36)         NOT NULL, 
    
    created_at              DATETIME        NOT NULL,
    
    report                  TEXT            NOT NULL
);