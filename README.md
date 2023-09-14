# sshc

> SSH Client to perform file copying and command execution

### Copying files to remote host

**Using private key**

```shell
./sshc exec --address 192.168.1.3 --username "{YOUR_USERNAME}" --private-key "{PATH_TO_PRIVATE_KEY}" --source "readme.txt" --destination "~/"
```

**Using password**

```shell
./sshc exec --address 192.168.1.3 --username "{YOUR_USERNAME}" --password "{YOUR_PASSWORD}" --source "readme.txt" --destination "~/"
```

### Execute command on remote host

**Using private key**

```shell
./sshc exec --address 192.168.1.3 --username "{YOUR_USERNAME}" --private-key "{PATH_TO_PRIVATE_KEY}" --command "ls -lA"
```

**Using password**

```shell
./sshc exec --address 192.168.1.3 --username "{YOUR_USERNAME}" --password "{YOUR_PASSWORD}" --command "ls -lA"
```