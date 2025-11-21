# Linux Stealth Rootkit Hunting with Command Line Forensics

Notes from https://www.youtube.com/watch?v=pZbEUHdwio8
Craig Rowland, Sandfly Security

- based on Reptile rootkit framework
- Chinese to South Korea infrastructure

Main takeways:

- Linux doesn't hide anything 
- Good forensic technique important
- Improved my understanding of how the Linux OS works
- Stealth rootkits not exactly the biggest threat
 
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

### World's Fastest Rootkit Confirmation

Make a file using `touch <suspected-filename>` 

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

Linx will show tainted kernels with designations such as:
- (P) - Proprietary
- (O) - Out of Tree
- (E) - Unsigned Module (DANGER)

All unsigned modules need to be closely inspected.

So you can use 

```
grep "(" /proc/modules
```

to see which modules are tainted and under what designation they are.

If `cat /proc/sys/kernel/tainted` flag value != 0, it means you have a tainted module.

Crosscheck - If you use `grep "(" /proc/modules` and no module shows, that's an 
inconsistency.

It's possible but not likely that a module has loaded itself, 
tainted the kernel, and then unloaded itself.

## Kernel Taint Check Script

Find out how the kernel has been tainted 

`./kernel-chktaint.sh`

More info:

https://docs.kernel.org/admin-guide/tainted-kernels.html

## Kernel Taint in Logs 

Check the logs, and ask in multiple ways:

```
grep taint /var/log/kern.log
```

```
dmesg | grep taint
```

```
journalctl -t kernel
journalctl -t kernel | grep taint
journalctl -t kernel | grep signature
```

## Kernel Module Decloak

`/proc/vmallocinfo` is real-time memory use at run-time on Linux for processes, 
including kernel modules.

```
grep "\[" /proc/mallocinfo
```
In the example, the hidden module `vmwfxs` also shows `khook_init` which is a known
kernel hook library call.

If you don't see the module after grep, it's hiding again.
```
grep <kernel_module_name> /proc/modules
```

Sandfly also has a decloaking script here:
https://github.com/sandflysecurity/sandfly-kernel-module-decloak

It will decloak and give info for you.

## Hidden Module Data

First, copy the binary to an isolated system before performing the following commands.

```
file /usr/lib64/tracker-fs
```
In the example, the file is not stripped, making analysis easier.

```
strings /usr/lib64/tracker-fs
```

Performing the `strings` command reveals at least the following:

- `kallsyms_lookup_name`, `khook: waiting for %s...`  Hooking system calls (kallsyms, khook)

- `tracker-fs` Backdoor binary name

- `filldir64`, `proc_root_readdir` Hiding directory entities (filldir/readdir)

- `tcp4_seq_show` TCP concealment for hiding or obfuscating network traffic

- `bash`, `cash` Shells for backdoor (bash, etc.)

- Various paths

- `httpch`, `smtp`, `https` Various protocols

- `acpi/pcicard` Control socket (/proc/acpi/pcicard)

- `HISTORY=/dev/null`, `BASH_HISTORY=/dev/null` Shell anti-forensics

- `vmwfxs` Module name

- `tracker-efs` Backdoor process name

- Various passwords used to authenticate against the backdoor

## The Backdoor

Strengths:
- Activates on any port with magic packet
- Enables lateral movement or shell access
- Has anti-forensics features

Weaknesses:
- Incomplete hiding exposes it
- Can recover binary when running
- Pressing attack always compromises stealth

## Magic Packet Backdoor Activation

## Backdoor Features

## Load Detection

Running scripts on a clean system will be noticeably faster than a dirty system

Hidden rootkits need to intercept everything happening in the system to stay hidden

Try running the same script 100 times on different systems

## Conclusion

- Stealth rootkits work, but they are fragile
- More visible when operator presses attack
- Older systems not closely watched can have rootkits, because no one is looking
- Craig shows how Sandfly scales and automates the methods presented





