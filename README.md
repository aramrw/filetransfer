### Usage
1. Run `exe`
2. Go to link on second device
3. Upload and Download 

### Example
```pwsh
Windows IPv4: ipconfig 
Linux IPv4: ip -4 addr 
```
#### Upload a file
```pwsh
$ curl -X POST -F "file=@./path/to/file.txt" http://<YOUR_IP>:3000/upload
```
