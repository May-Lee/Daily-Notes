# Linux Stealth Rootkit Hunting with Command Line Forensics

Notes from https://www.youtube.com/watch?v=pZbEUHdwio8

- based on Reptile rootkit framework
- Chinese to South Korea infrastructure

Main takeways:

- Linux doesn't hide anything 
- Good forensic technique important
- Improved my understanding of how the Linux OS works
 
## Basic Concepts:

- Common Stealth Rootkit Features
  - Hides Module
  - Hides Files
  - Hides Directories
  - Hides Processes
  - Magic Packet Activation
    - Sent to system to activate Backdoor
  - Evasive Backdoor

- "Juggling Hand Grenades"
 - Advantages
   - Can hide from casual detection
   - can evade some focused detection efforts
   - have stealthy backdoor cabilities
 - Disadvantages
   - Cause system stability impacts
   - Fragile and tied to specific kernel versions
   - Obvious once seen

- Principles of Linux Rootkit Hunting
  - Data leaks
  - Inconsistent Answers
  - System Impacts
   - for stealth rootkits, they impact function

## Methods:

### Looking for data leaks

After initialization scripts, rootkit will hide modules.

Method: 
Ask `systemd` what init scripts it has run

```
 systemctl list-units --state=exited
 systemctl status <hidden-tracker-fs.service>

```

In the video example, the path `/etc/init.d/tracker-fs` is leaked

But `ls -al` doesn't show it in init.d; it is missing

REMEMBER, Linux doesn't hide anything, so use forensic principle #2

### Ask multiple times and look for inconsistent responses

Some commands to find a file/file path

```

ls -al
stat <file-name>
cat <file-name> #as we'll see later, data within files can be hidden
file <file-name>

```

In the example, `stat <hidden-tracker-fs>` shows the file. 

Using `cat <hidden-tracker-fs>` will show the contents of the file,
t=1293
BUT

### World's Fastest Rootkit Confirmation

Make a file using the <suspected-filename> 

If it's not in the directory listing afterwards, Rootkit Active!
 
Call Incident Response, it's time for a memory dump!

### Decloaking Hidden File Data

If you use `cat <hidden-tracker-fs>` there might be some file data that's been hidden

Using `vi` or an editor might still not reveal the hidden data

You can use byte count mismatch. Run the following commands and compare output

```
ls -al /etc/modules
cat /etc/modules | wc -c
```
The kernel can lie, but the file system won't

### Single Byte Read Decloaking

Fragment the file into single bytes to bypass rootkit buffer.
Rootkits won't reassemble the text due to cost.

```
grep . /path/to/file 
dd count=10000 bs=1 if=/path/to/file 2>/dev/null
```

## Tainted Kernels







