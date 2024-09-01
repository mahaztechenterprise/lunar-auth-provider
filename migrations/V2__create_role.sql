
CREATE TABLE APP_ROLE(
    id CHAR(36) PRIMARY KEY NOT NULL,
    role_name VARCHAR(255) NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


CREATE TABLE APP_USER_ROLE (
    id CHAR(36) PRIMARY KEY NOT NULL,
    user_id CHAR(36) NOT NULL,
    role_id CHAR(36) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES APP_USER(id),
    FOREIGN KEY (role_id) REFERENCES APP_ROLE(id)
);