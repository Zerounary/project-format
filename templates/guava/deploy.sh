#! /bin/bash
cd /home/ubuntu/{{{project_name}}}
app="{{{project_name}}}"
# kill server pid if exists
if [ -f $app.pid ]; then
    sudo kill -9 $(cat $app.pid)
    rm -f $app.pid
fi

# 避免nohup命令不终止
echo "restart server"
sudo nohup ./$app >/dev/null 2>&1 &
echo $! > $app.pid
echo "restart server done"