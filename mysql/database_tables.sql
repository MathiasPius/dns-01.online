CREATE TABLE `users` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(100) NOT NULL,
  `passhash` varchar(255) NOT NULL,
  `salt` varchar(255) NOT NULL,
  `apikey` varchar(32) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `users_UN` (`username`),
  UNIQUE KEY `apikey_UN` (`apikey`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4

CREATE TABLE `tokens` (
  `name` varchar(63) NOT NULL,
  `token` varchar(128) NOT NULL,
  `expiration` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `apikey` varchar(32) NOT NULL,
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`name`),
  UNIQUE KEY `tokens_UN` (`id`),
  KEY `tokens_users_FK` (`apikey`),
  CONSTRAINT `tokens_users_FK` FOREIGN KEY (`apikey`) REFERENCES `users` (`apikey`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4
