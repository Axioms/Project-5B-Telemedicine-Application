-- Your SQL goes here
CREATE TABLE reports ( 
    uuid                    CHAR(36)         NOT NULL PRIMARY KEY, 
    patient_uuid            CHAR(36)         NOT NULL, 
    provider_uuid           CHAR(36)         NOT NULL, 

    created_at              DATETIME        NOT NULL,
    
    report                  TEXT            NOT NULL
);