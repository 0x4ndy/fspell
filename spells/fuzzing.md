# Web files

### Web file extensions fuzzing with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/<file_name>FUZZ
```
---

# Web directories

### Directory fuzzing with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/FUZZ
```
---