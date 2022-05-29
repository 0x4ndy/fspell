# DNS

### one-liner forward lookup brute force
```bash
for ip in $(cat <file-name.txt>); do host $ip.<domain>; done | grep -v "not found"
```
---
### one-liner reverse lookup brute force
```bash
for ip in $(sec 1 254); do host <aaa.bbb.ccc>.$ip; done | grep -v "not found"
```
---
### List DSN servers
```bash
host -t ns <domain_name> | cut -d " " -f 4
```
---
### DNS zone transfer
```bash
host -l <domain_name> <dns_server_address>
```
---
### dnsrecon zone transfer
```bash
dnsrecon -d <domain_name> -t axfr
```
---
### dnsrecon brute force
```bash
dnsrecon -d <domain_name> -D <word_list> -t brt
```
---
# Ports

### TCP CONNECT scanning using netcat
```bash
nc -nvv -w <timeout_seconds> -z <ip> <port/port_range>
```
---
### UDP scanning using netcat
```bash
nc -nv -u -z -w <timeout_seconds> <ip> <port/port_range>
```
---
### Top N ports TCP with NMAP
```bash
sudo nmap --top-ports <num_ports> -sC -sV <ip> --open --reason
```
---
### Full port range TCP (with banner grabbing and default scripts) using NMAP
```bash
sudo nmap -p- -sC -sV <ip> --open --reason
```
---
### Network sweep using NMAP
```bash
nmap -v -sn <aaa.bbb.ccc.1-254> -oG <file_name>
```
---
### Network top N port sweep with OS, script and traceroute enabled, using NMAP
```bash
nmap -sT -A --top-ports=<num_ports> <aaa.bbb.ccc.1-254> -oG <file_name>
```
---
# SMB

### SMB scan using NMAP
```bash
nmap -v -p 139,445 -oG <output_file> <ip/ip_range>
```
---
### SMB scan using nbtscan
```bash
sudo nbtscan -r <aaa.bbb.ccc.0/24>
```
---
### SMB scan using NMAP scripts
```bash
nmap -v -p 139,445 --script=smb-* <ip>
```
---