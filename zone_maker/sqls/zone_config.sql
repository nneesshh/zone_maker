/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 50726
 Source Host           : localhost:3306
 Source Schema         : test_gpaas

 Target Server Type    : MySQL
 Target Server Version : 50726
 File Encoding         : 65001

 Date: 10/01/2024 10:33:31
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for zone_config
-- ----------------------------
DROP TABLE IF EXISTS `zone_config`;
CREATE TABLE `zone_config`  (
  `zone_id` bigint(20) UNSIGNED NOT NULL COMMENT '小区id',
  `group_id` int(11) NULL DEFAULT NULL COMMENT '平台id',
  `gw_port` int(11) NULL DEFAULT NULL COMMENT 'gw端口',
  `http_port` int(11) NULL DEFAULT 0 COMMENT 'http端口',
  `db_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '数据库连接 url',
  `redis_ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT 'redis ip',
  `redis_port` int(11) NULL DEFAULT NULL COMMENT 'redis port',
  `web_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT '0' COMMENT 'web上报地址',
  `test_blob` tinyblob NULL COMMENT '测试 blob',
  `test_timestamp` datetime NULL DEFAULT NULL COMMENT '测试 timestamp',
  `first_open_time` datetime NULL DEFAULT NULL COMMENT '开服日期',
  `merge_list` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '合服列表',
  `merge_times` int(11) NULL DEFAULT NULL COMMENT '合服次数',
  `merge_date` datetime NULL DEFAULT NULL COMMENT '合服日期',
  `local_public_ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '外网ip',
  `local_private_ip` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NULL DEFAULT NULL COMMENT '内网ip',
  PRIMARY KEY (`zone_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = Dynamic;

-- ----------------------------
-- Records of zone_config
-- ----------------------------
INSERT INTO `zone_config` VALUES (5001, 4, 20001, 8888, 'mysql://db_url', 'localhost', 443, 'http://web_url', 0x626C61626C61626C61, '2023-12-29 06:20:42', '2023-12-29 06:20:42', NULL, NULL, NULL, NULL, NULL);
INSERT INTO `zone_config` VALUES (5002, 4, 20001, 8888, 'mysql://db_url', 'localhost', 443, 'http://web_url', 0x626C61626C61626C61, '2023-12-29 06:20:42', '2023-12-29 07:20:42', NULL, NULL, NULL, NULL, NULL);

SET FOREIGN_KEY_CHECKS = 1;
