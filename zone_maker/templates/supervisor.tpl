;[unix_http_server]
;file=/tmp/supervisor.sock   ; UNIX socket 文件，supervisor ctl 会使用

;[supervisorctl]
;serverurl=unix:///tmp/supervisor.sock ;  ;连接到supervisor

[inet_http_server]
port=0.0.0.0:9001
;username=test1
;password=thepassword

[supervisorctl]
serverurl=http://127.0.0.1:9001

[supervisord]
logfile= /data/logs/supervisord/supervisord.log 	       ; 日志文件，默认是 $CWD/supervisord.log
logfile_maxbytes=50MB        ; 日志文件大小，超出会 rotate，默认 50MB
logfile_backups=10               ; 日志文件保留备份数量默认 10
loglevel=info                         ; 日志级别，默认 info，其它: debug,warn,trace
pidfile=supervisord.pid         ; pid 文件
nodaemon=false                  ; 是否在前台启动，默认是 false，即以 daemon 的方式启动
minfds=1024                         ; 可以打开的文件描述符的最小值，默认 1024
minprocs=200                       ; 可以打开的进程数的最小值，默认 200

[program-default]
directory = /opt/app/longzhu-overseas-game/bin         			; 程序的启动目录
environment=LD_LIBRARY_PATH="/opt/app/longzhu-overseas-game/bin:$LD_LIBRARY_PATH",LD_PRELOAD="libjemalloc.so.2"
autostart = true            				; 在 supervisord 启动的时候也自动启动
startsecs = 5               				; 启动 5 秒后没有异常退出，就当作已经正常启动了
autorestart = unexpected    			; 程序异常退出后自动重启
startretries = 3            				; 启动失败自动重试次数，默认是 3
restart_times=3					; 重启次数,超过3次之后不在自动重启
restart_delay=10					; 重启延迟,第一次立刻重启,第二次10s,没重启一次多等待10s
restart_reset_interval=3600			; 启动1个小时内没有down机的话,重置已重启次数(重新从0开始计数)
user = kingnet               				; 用哪个用户启动
redirect_stderr = true      				; 把 stderr 重定向到 stdout，默认 false
stdout_logfile_maxbytes = 20MB  		; stdout 日志文件大小，默认 50MB
stdout_logfile_backups = 20     			; stdout 日志文件备份数




[program:node_gatewayserver_1001]
command=./gateway_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s gatewayserver  -n 1001 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_gatewayserver_1001_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 100              ;  启动优先级,越小越先启动





[program:node_crossserver_1002]
command=./cross_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s crossserver  -n 1002{{ serverid }} ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_crossserver_1002_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 40              ;  启动优先级,越小越先启动





[program:node_gameserver_1003]
command=./game_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s gameserver  -n 1003 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_gameserver_1003_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 90              ;  启动优先级,越小越先启动






[program:node_dbserver_1004]
command=./dbserver -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s dbserver  -n 1004 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_dbserver_1004_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 70              ;  启动优先级,越小越先启动






[program:node_battleserver_1005]
command=./battle_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s battleserver  -n 1005 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_battleserver_1005_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 70              ;  启动优先级,越小越先启动






[program:node_worldserver_1006]
command=./world_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s worldserver  -n 1006 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_worldserver_1006_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 50              ;  启动优先级,越小越先启动
stopsignal=USR1             					   ;  world server收到 USR1走正常关服流程
stopwaitsecs=180								   ;  最多等3分钟还没正常关闭的话,直接kill掉






[program:node_commserver_1008]
command=./comm_server -g {{ platform_id }} -z {{ serverid }} -c /opt/app/etc/dragon_{{ serverid }}.xml -s commserver  -n 1008 ; 启动命令
process_name=%(program_name)s
stdout_logfile=/data/logs/supervisord/district_{{ serverid }}_{{ platform_id }}_node_commserver_1008_stdout.log 		   ;  stdout 日志文件，需要注意当指定目录不存在时无法正常启动，所以需要手动创建目录（supervisord 会自动创建日志文件）
priority = 80              ;  启动优先级,越小越先启动




