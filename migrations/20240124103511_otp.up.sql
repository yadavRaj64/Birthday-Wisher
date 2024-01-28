-- Add up migration script here
CREATE TABLE otps (
    otp VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_for VARCHAR(255) NOT NULL,
    used BOOLEAN NOT NULL,
    sent BOOLEAN NOT NULL
);
