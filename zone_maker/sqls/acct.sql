CREATE TABLE `account_2009` (
  `account` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '游戏账号',
  `sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'sdkuid',
  `rebated` tinyint(1) DEFAULT '0' COMMENT '是否返利',
  `bind_channel` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定渠道',
  `bind_account` bigint DEFAULT NULL COMMENT '绑定账号',
  `bind_sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定sdkuid',
  `bind_reward` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否领取绑定奖励',
  PRIMARY KEY (`account`) USING BTREE,
  UNIQUE KEY `UID_IDX` (`sdk_uid`) USING BTREE,
  KEY `ACCT_IDX` (`bind_account`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2009100000000 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `account_2010` (
  `account` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '游戏账号',
  `sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'sdkuid',
  `rebated` tinyint(1) DEFAULT '0' COMMENT '是否返利',
  `bind_channel` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定渠道',
  `bind_account` bigint DEFAULT NULL COMMENT '绑定账号',
  `bind_sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定sdkuid',
  `bind_reward` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否领取绑定奖励',
  PRIMARY KEY (`account`) USING BTREE,
  UNIQUE KEY `UID_IDX` (`sdk_uid`) USING BTREE,
  KEY `ACCT_IDX` (`bind_account`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2010100000000 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `account_2011` (
  `account` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '游戏账号',
  `sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'sdkuid',
  `rebated` tinyint(1) DEFAULT '0' COMMENT '是否返利',
  `bind_channel` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定渠道',
  `bind_account` bigint DEFAULT NULL COMMENT '绑定账号',
  `bind_sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定sdkuid',
  `bind_reward` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否领取绑定奖励',
  PRIMARY KEY (`account`) USING BTREE,
  UNIQUE KEY `UID_IDX` (`sdk_uid`) USING BTREE,
  KEY `ACCT_IDX` (`bind_account`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2011100000000 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `account_2012` (
  `account` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '游戏账号',
  `sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'sdkuid',
  `rebated` tinyint(1) DEFAULT '0' COMMENT '是否返利',
  `bind_channel` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定渠道',
  `bind_account` bigint DEFAULT NULL COMMENT '绑定账号',
  `bind_sdk_uid` varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '绑定sdkuid',
  `bind_reward` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否领取绑定奖励',
  PRIMARY KEY (`account`) USING BTREE,
  UNIQUE KEY `UID_IDX` (`sdk_uid`) USING BTREE,
  KEY `ACCT_IDX` (`bind_account`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=2012100000000 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;