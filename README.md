### Browser Usage
1. Run `exe`
2. Go to `http://<YOUR_IP>::<PORT>/` on second device 👍

### Cli Example
**_Win_: `ipconfig`
_Linux_: `ip -4 addr`**

#### Upload a file
```pwsh
$ curl -X POST -F "file=@./path/to/file.txt" http://<YOUR_IP>:3000/upload
```
#### Download a file
```pwsh
$ curl -O http://<YOUR_IP>:3000/download/file/<FILE_NAME>
```
