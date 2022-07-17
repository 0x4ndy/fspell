# Web files

### Web file extensions fuzzing with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/<file_name>FUZZ
```
---

### Web file names fuzzing with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/FUZZ.<extension>
```
---

# Web directories

### Directory fuzzing with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/FUZZ
```
---

### Recursive (files/directories) Scanning with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http://<ip>:<port>/FUZZ -recursion -recursion-depth 1 -e <file-extension> -v
```
---

# Sub-domains

### Fuzzing sub-domains with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://FUZZ.<domain-name>
```
---

# Vhosts

### Fuzzing Vhosts with FFuf
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://<domain>:<port>/ -H 'Host: FUZZ.<domain>'
```
---

# Parameters

### Fuzzing GET parameters with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://<domain>:<port>/<path>?FUZZ=key
```
---

### Fuzzing POST parameters with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://<domain>:<port>/<path> -X POST -d 'FUZZ=key' -H 'Content-Type: application/x-www-form-urlencoded'
```
---

### Fuzzing GET parameters value with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://<domain>:<port>/<path>?<parameter-name>=FUZZ
```
---

### Fuzzing POST parameters value with FFuF
```bash
ffuf -w <wordlist>:FUZZ -u http(s)://<domain>:<port>/<path> -X POST -d '<param-name>=FUZZ' -H 'Content-Type: application/x-www-form-urlencoded'
```
---