-- Add migration script here

SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS module;
DROP TABLE IF EXISTS module_version;
DROP TABLE IF EXISTS module_owner;


CREATE TABLE module_owner (
    module_owner_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    module_owner_name VARCHAR(255) NOT NULL
);

CREATE TABLE module (
    module_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    module_name VARCHAR(255) NOT NULL,
    env JSON,
    module_owner_id INT(11) UNSIGNED NOT NULL,
    module_version_id INT(11) UNSIGNED DEFAULT NULL,

    CONSTRAINT FK_module_owner_id FOREIGN KEY (module_owner_id) REFERENCES module_owner(module_owner_id),
    CONSTRAINT FK_module_version_id FOREIGN KEY (module_version_id) REFERENCES module_version (module_version_id) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE module_version (
    module_version_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    hash_value VARCHAR(255) NOT NULL,
    raw_value BINARY(255) NOT NULL,
    pre_compiled BINARY(255) DEFAULT NULL,
    module_id INT(11) UNSIGNED NOT NULL,

    CONSTRAINT FK_module_id FOREIGN KEY (module_id) REFERENCES module(module_id) ON DELETE CASCADE ON UPDATE NO ACTION
);
SET FOREIGN_KEY_CHECKS=1;
