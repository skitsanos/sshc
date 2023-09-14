# sshc

> SSH Client to perform file copying and command execution

This Rust-based application is a command-line SSH client tailored for two specific operations: copying files remotely using the SCP protocol (`copy` operation) and executing commands on remote machines (`exec` operation). 

The application supports the following functionalities:

### Copy Operation (Copy CLI):

- Copies files from a local machine to a remote location using SCP.
- Requires `address`, `username`, `source`, and `destination` paths as mandatory arguments.
- Supports optional authentication through either a password or a private key.
- It can provide outputs in a JSON format.

### Remote Execution Operation (Exec CLI):

- Executes commands on a remote machine via SSH.
- Requires address, username, and command as mandatory arguments.
- Supports optional authentication through either a password or a private key.
- For establishing a secure connection, the application offers flexible authentication modes. Users can either use a password or a private key to authenticate their session, ensuring broader compatibility with various SSH server configurations.

### Utility in CI/CD operations:

This application holds significant value for Continuous Integration and Continuous Deployment (CI/CD) operations, particularly for teams and platforms that leverage multiple operating systems like OSX, Linux, and Windows:

**Cross-Platform Compatibility**: With support for OSX, Linux, and Windows, the application can be seamlessly integrated into diverse CI/CD pipelines, regardless of the underlying operating system.

**Automated Deployments**: The Copy operation can be leveraged to deploy build artifacts from a CI server to staging or production servers. With the flexible authentication options, it can easily be plugged into automated scripts or pipelines.

**Remote Command Execution**: CI/CD operations often require a series of commands to be executed on remote servers, such as starting/stopping services, database migrations, or clearing caches. The Exec operation caters to this need.

**Flexible Authentication**: Given that CI/CD operations need to maintain security, the ability to use private keys for authentication ensures that scripts can operate without exposing passwords.

**Structured Output with JSON**: For CI/CD tools that consume structured output (like JSON) for further processing or logging, the application provides an option to output in the JSON format. This aids in better logging, monitoring, and alerting in CI/CD pipelines.

**Standardization**: Instead of relying on different tools or scripts for different platforms, organizations can standardize their CI/CD operations using this application, reducing complexity and potential points of failure.

## Examples

### Copying files to remote host

**Using private key**

```shell
./sshc exec --address "{SERVER_ADDRESS}" --username "{USERNAME}" --private-key "{PATH_TO_PRIVATE_KEY}" --source "readme.txt" --destination "~/"
```

**Using password**

```shell
./sshc exec --address "{SERVER_ADDRESS}" --username "{USERNAME}" --password "{YOUR_PASSWORD}" --source "readme.txt" --destination "~/"
```

### Execute command on remote host

**Using private key**

```shell
./sshc exec --address "{SERVER_ADDRESS}" --username "{SERNAME}" --private-key "{PATH_TO_PRIVATE_KEY}" --command "ls -lA"
```

**Using password**

```shell
./sshc exec --address "{SERVER_ADDRESS}" --username "{USERNAME}" --password "{PASSWORD}" --command "ls -lA"
```