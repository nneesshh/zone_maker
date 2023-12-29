<?xml version="1.0" encoding="gb2312"?>
<config zone="{{ config.zone_id }}" group="{{ config.group_id }}">
  <nodes>1001,1002,1003,1004,1005,1006,1008</nodes>
  <!--开服时间,正式环境从DB配置读取-->
  <open_time>2023-09-26 00:30:00</open_time>
  <!--跨服逻辑服务器-->
  <cross>{{ concat 1002 config.zone_id }}</cross>
  <!--跨服社交玩法服务器节点-->
  <!--跨服协会服务器节点-->
  <guild>1012500</guild>
  <!--跨服包含的区-->
  <zones>5001,5002,5003,5004,5005,5006,5007</zones>
  <!--合并的区-->
  <merges>5002,5003,5004,5005,5006,5007</merges>
  <!--合服次数-->
  <merge_times>1</merge_times>
  <!--合服时间-->
  <merge_date>2023-10-23 08:39:04</merge_date>
  <!--是否检查登录合服-->
  <merge_check>1</merge_check>
  <!--是否检查登录session-->
  <session_check>true</session_check>
  <!--服务器版本号-->
  <version check="false">v1.14.5</version>
  <!--使用语言  English/Chinese-->
  <language>English</language>
  <!--使用时区 东1-12区:(1)-(12) 西1-12区:(-1)-(-12) -->
  <time_zone>-3</time_zone>
  <!--http服务端口号(用来接收gm命令/充值消息/提供监控数据)-->
  <http_port>8888</http_port>
  <!--是否开启gm命令-->
  <gm>false</gm>
  <!--本机公网ip-->
  <local_public_ip>43.157.137.23</local_public_ip>
  <!--本机内网ip-->
  <local_private_ip>10.40.0.48</local_private_ip>
  <!--16字节加密密钥(BASE64编码)-->
  <!--<encrypt_token>weQX+iuRHndfnc41XqtBTg==</encrypt_token>-->
  <encrypt_token>YWJjZGVmZ2hpamtsbW5vcA==</encrypt_token>
  <!--web服务地址-->
  <web>http://10.40.0.4/</web>
  <!--java推送服务地址（不使用）-->
  <web_new></web_new>
  <!--获取新玩家可用PID-->
  <!--限制玩家上限数-->
  <limit_players>3000000</limit_players>

  <player_ids_url>inner/takepids</player_ids_url>
  <guild_ids_url>inner/takeguildids</guild_ids_url>
  <player_update_url>inner/playerupdate</player_update_url>
  <nodes_url>inner/game_config</nodes_url>
  <login_check_url>inner/login</login_check_url>
  <pay_rebate_url>inner/rebate</pay_rebate_url>
  <guild_update_url>inner/guildupdate</guild_update_url>
  <!--兑换码-->
  <redeem_code_url>http://10.40.0.10:9998/gift/openApi/exchange</redeem_code_url>
  <!--上传区服状态-->
  <server_status_url>http://10.40.0.10:9998/game/openApi/server/status</server_status_url>
  <!--多语言埋点日志上报-->
  <multi_log_url>inner/multi_log</multi_log_url>
  <!--兑换码-->（不使用）
  <new_redeem_code_url></new_redeem_code_url>
  <!--上报玩家信息-->
  <players_report_url>http://10.40.0.10:9995/longzhu/roles</players_report_url>
  <!--推送服务器地址，待定，先留空-->
  <push_url></push_url>
  <!--聊天监控-->
  <chat_supervisory_url>inner/chatsupervisory</chat_supervisory_url>
  <!--获取GPaaS上的配置-->
  <gpaas_zone_config_url>inner/get_gpaas_zone_config</gpaas_zone_config_url>
  <!--获取跨服玩法关系的配置信息-->
  <cross_relation_config_url>inner/cross_relation_config</cross_relation_config_url>
  <!--创角上限-->
  <limit_players>20000</limit_players>
  <!--上传性能数据的httpurl-->
  <metrics></metrics>
  <!--16字节加密密钥(BASE64编码)-->
  <encrypt_token>YWJjZGVmZ2hpamtsbW5vcA==</encrypt_token>
  <!--pid文件保存目录-->
  <pid path="/data/logs/longzhu-overseas-game/"></pid>
  <!--日志配置-->
  <log queue="8192" threads="4" path="/data/logs/longzhu-overseas-game/" bi="bi" level="1" console="false"></log>
  <!--gm接口配置-->
  <gm_config appid="10" appkey="5aSrMjP6Dlj6lpYjkvZXoirHokL3ljrsyM2RkczNn"></gm_config> 

  <!--平台缓存配置-->
  <redis>
    <db addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="1"></db>
    <queue addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="2"></queue>
  </redis>

  <!-- gateway -->
  <node srv="1001" id="1001">
    <addr>0.0.0.0</addr>
    <port>20001</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <game id="1003" addr="127.0.0.1" port="20003"></game>
    <battle id="1005" addr="127.0.0.1" port="20005"></battle>
    <db id="1004" addr="127.0.0.1" port="20004"></db>
    <common id="1008" addr="127.0.0.1" port="20008"></common>
    <player_limit_msg>区服角色已满</player_limit_msg>
  </node>
  <!-- game -->
  <node srv="1003" id="1003">
    <addr>127.0.0.1</addr>
    <port>20003</port>
    <battle id="1005" addr="127.0.0.1" port="20005"></battle>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <db id="1004" addr="127.0.0.1" port="20004"></db>
    <log id="1009" addr="127.0.0.1" port="20009"></log>
    <common id="1008" addr="127.0.0.1" port="20008"></common>
  </node>
  <!-- db -->
  <!-- <node srv="1004" id="1004" limit_players="5000"> -->
  <node srv="1004" id="1004">
    <addr>127.0.0.1</addr>
    <port>20004</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <game>
      <addr>10.40.0.14</addr>
      <port>3306</port>
      <user>ms_ser</user>
      <pwd>ymUYNEC^M_wwx5De</pwd>
      <db>moshen_zone_s5001_m1</db>
    </game>
  </node>
  <!-- battle -->
  <node srv="1005" id="1005">
    <addr>127.0.0.1</addr>
    <port>20005</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
  </node>
  <!-- world -->
  <node srv="1006" id="1006">
    <addr>0.0.0.0</addr>
    <port>20006</port>
    <nodes>1001,1003,1004,1005,1008</nodes>
    <redis>
      <db addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="1"></db>
      <queue addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="2"></queue>
    </redis>
  </node>
  <!-- commonserver -->
  <node srv="1008" id="1008">
    <addr>127.0.0.1</addr>
    <port>20008</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <db id="1004" addr="127.0.0.1" port="20004"></db>
    <log id="1009" addr="127.0.0.1" port="20009"></log>
  </node>
  <!-- cross -->
  <node srv="1002" id="10025001">
    <addr>127.0.0.1</addr>
    <port>20002</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <redis>
      <db addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="1"></db>
      <queue addr="10.40.0.22" port="6379" auth="jawa3776RDGPdldz" db="2"></queue>
    </redis>
  </node>
  <!-- chat -->
  <node srv="1010" id="1010" name="chatsrv">
    <addr>127.0.0.1</addr>
    <port>20010</port>
    <world id="1006" addr="127.0.0.1" port="20006"></world>
    <db id="1004" addr="127.0.0.1" port="20004"></db>
    <log id="1009" addr="127.0.0.1" port="20009"></log>
  </node>

</config>
