#! /bin/bash  
cd /home/app  
app="{{project_name}}-0.0.1-SNAPSHOT.jar"  
# kill server pid if exists  
if [ -f $app.pid ]; then  
    kill -9 $(cat $app.pid)  
    rm -f $app.pid  
fi  
  
# 避免nohup命令不终止  
echo "restart server"  
nohup java -jar ./$app >/dev/null 2>&1 &  
echo $! > $app.pid  
echo "restart server done"