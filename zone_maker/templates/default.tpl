<?xml version="1.0" encoding="gb2312"?>
<config zone="{{ serverid }}" group="{{ platform_id }}">
  <!--开服时间,正式环境从DB配置读取-->
  <open_time>{{ first_open_time }}</open_time>
  <!--跨服逻辑服务器-->
  <cross>1002{{ serverid }}</cross>
  <!--跨服协会服务器节点-->
  <guild>1012{{ serverid }}</guild>
  <!--跨服战斗服务器节点-->
  <battle>1013{{ serverid }}</battle>
  <!--跨服包含的区-->
  <zones>{{ merge_list }}</zones>
  <!--合并的区-->
  <merges>{{ merge_list }}</merges>
  <!--合服次数-->
  <merge_times>{{ merge_times }}</merge_times>
  <!--合服时间-->
  <merge_date>{{ merge_date }}</merge_date>
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
  <http_port>{{ http_port }}</http_port>
  <!--是否开启gm命令-->
  <gm>false</gm>
  <!--本机公网ip-->
  <local_public_ip>{{ local_public_ip }}</local_public_ip>
  <!--本机内网ip-->
  <local_private_ip>{{ local_private_ip }}</local_private_ip>
  <!--16字节加密密钥(BASE64编码)-->
  <encrypt_token>YWJjZGVmZ2hpamtsbW5vcA==</encrypt_token>
  <!--web服务地址-->
  <web>{{ web_ip }}</web>
  <!--限制玩家上限数-->
  <limit_players>20000</limit_players>
  <!--获取新玩家可用PID-->
  <player_ids_url>inner/takepids</player_ids_url>
  <!--获取新公会PID-->
  <guild_ids_url>inner/takeguildids</guild_ids_url>
  <!--更新玩家基本信息-->
  <player_update_url>inner/playerupdate</player_update_url>
  <!--登录验证url地址-->
  <login_check_url>inner/login</login_check_url>
  <!--充值返利-->
  <pay_rebate_url>inner/rebate</pay_rebate_url>
  <!--更新公会基本信息-->
  <guild_update_url>inner/guildupdate</guild_update_url>
  <!--多语言埋点日志上报-->
  <multi_log_url>inner/multi_log</multi_log_url>
  <!--聊天监控-->
  <chat_supervisory_url>inner/chatsupervisory</chat_supervisory_url>
  <!--获取跨服玩法关系的配置信息-->
  <cross_relation_config_url>inner/cross_relation_config</cross_relation_config_url>
  <!--兑换码-->
  <redeem_code_url>http://{{ gm_ip }}:9998/gift/openApi/exchange</redeem_code_url>
  <!--上传区服状态-->
  <server_status_url>http://{{ gm_ip }}:9998/game/openApi/server/status</server_status_url>
  <!--上报玩家信息-->
  <players_report_url>http://{{ gm_ip }}:9995/longzhu/roles</players_report_url>
  <!--gm接口配置-->
  <gm_config appid="10" appkey="5aSrMjP6Dlj6lpYjkvZXoirHokL3ljrsyM2RkczNn"></gm_config> 
  <!--推送服务器地址，待定，先留空-->
  <push_url></push_url>
  <!--16字节加密密钥(BASE64编码)-->
  <encrypt_token>YWJjZGVmZ2hpamtsbW5vcA==</encrypt_token>
  <!--pid文件保存目录-->
  <pid path="/data/logs/{{ name_en }}/"></pid>
  <!--日志配置-->
  <log queue="8192" threads="4" path="/data/logs/{{ name_en }}/" bi="bi" level="1" console="false"></log>

  <!--平台缓存配置-->
  <redis>
    <db addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="1"></db>
    <queue addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="2"></queue>
  </redis>

  <!-- gateway -->
  <node srv="1001" id="1001">
    <addr>0.0.0.0</addr>
    <port>{{ gateway_port }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <game id="1003" addr="127.0.0.1" port="{{ add gateway_port 2 }}"></game>
    <db id="1004" addr="127.0.0.1" port="{{ add gateway_port 3 }}"></db>
    <common id="1008" addr="127.0.0.1" port="{{ add gateway_port 5 }}"></common>
    <player_limit_msg>区服角色已满</player_limit_msg>
  </node>
  <!-- cross -->
  <node srv="1002" id="1002{{ serverid }}">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 1 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <redis>
      <db addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="1"></db>
      <queue addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="2"></queue>
    </redis>
  </node>
  <!-- game -->
  <node srv="1003" id="1003">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 2 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <db id="1004" addr="127.0.0.1" port="{{ add gateway_port 3 }}"></db>
    <log id="1009" addr="127.0.0.1" port="00000"></log>
    <common id="1008" addr="127.0.0.1" port="{{ add gateway_port 5 }}"></common>
  </node>
  <!-- db -->
  <node srv="1004" id="1004">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 3 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <game>
      <addr>{{ db_host }}</addr>
      <port>3306</port>
      <user>{{ db_user }}</user>
      <pwd>{{ db_password }}</pwd>
      <db>{{ db_name }}</db>
    </game>
  </node>
  <!-- world -->
  <node srv="1006" id="1006">
    <addr>0.0.0.0</addr>
    <port>{{ add gateway_port 4 }}</port>
    <nodes>1001,1003,1004,1008</nodes>
    <redis>
      <db addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="1"></db>
      <queue addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="2"></queue>
    </redis>
  </node>
  <!-- commonserver -->
  <node srv="1008" id="1008">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 5 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <db id="1004" addr="127.0.0.1" port="{{ add gateway_port 3 }}"></db>
    <log id="1009" addr="127.0.0.1" port="00000"></log>
  </node>
  <!-- chat -->
  <node srv="1010" id="1010" name="chatsrv">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 6 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <db id="1004" addr="127.0.0.1" port="{{ add gateway_port 3 }}"></db>
    <log id="1009" addr="127.0.0.1" port="00000"></log>
  </node>
  <!-- guild -->
  <node srv="1012" id="1012{{ serverid }}">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 7 }}</port>
    <world id="1006" addr="127.0.0.1" port="{{ add gateway_port 4 }}"></world>
    <db id="1004" addr="127.0.0.1" port="{{ add gateway_port 3 }}"></db>
    <redis>
      <db addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="1"></db>
      <queue addr="{{ redis_ip }}" port="6379" auth="{{ redis_password }}" db="2"></queue>
    </redis>
  </node>
  <!-- battle -->
  <node srv="1013" id="1013{{ serverid }}">
    <addr>127.0.0.1</addr>
    <port>{{ add gateway_port 8 }}</port>
  </node>
</config>
