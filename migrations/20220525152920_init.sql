-- Add migration script here

SET FOREIGN_KEY_CHECKS=0;
DROP TABLE IF EXISTS module;
DROP TABLE IF EXISTS module_version;
DROP TABLE IF EXISTS module_owner;


CREATE TABLE module_owner (
    owner_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    owner_name VARCHAR(255) NOT NULL
);

CREATE TABLE module (
    module_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    module_name VARCHAR(255) NOT NULL,
    module_env JSON DEFAULT NULL,
    owner_id INT(11) UNSIGNED NOT NULL,
    version_id INT(11) UNSIGNED DEFAULT NULL,

    CONSTRAINT FK_module_owner_id FOREIGN KEY (owner_id) REFERENCES module_owner(owner_id),
    CONSTRAINT FK_module_version_id FOREIGN KEY (version_id) REFERENCES module_version (version_id) ON DELETE SET NULL ON UPDATE CASCADE,
    CHECK (JSON_VALID(module_env))
);

CREATE TABLE module_version (
    version_id INT(11) UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    version_digest_value VARCHAR(255) NOT NULL,
    version_raw_value LONGBLOB NOT NULL,
    module_id INT(11) UNSIGNED NOT NULL,

    CONSTRAINT FK_module_id FOREIGN KEY (module_id) REFERENCES module(module_id) ON DELETE CASCADE ON UPDATE NO ACTION
);
SET FOREIGN_KEY_CHECKS=1;
