# Step to run find error logs on linux
Go to $VSCOMMON/webroot/logs/
```bash
find . -type f -newermt $(date -d 'yesterday' +'%Y-%m-%d') -exec grep -nEw -A1 "ERROR|ERR|ER" {} + > /export/_error$(date +'%Y%m%d').log
```

## requirements 
- Write a program that can run similarly with above comannd 
- Can run both on ```linux``` and ```windows```
- The program is written by **Rust** programming language