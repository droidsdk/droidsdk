# Install DSDK on Linux

You will need Rust to compile DSDK. This is temporary. In future, binaries will be distributed for all major systems.

## Download and install DSDK

Clone the github repository into the folder you wish to use as your installation dir. Execute ``cargo build`` in that
directory.

## Edit .bashrc

Open ``~/.bashrc`` (or the respective startup script for your shell) and add the following lines

```shell script
[[ -s "${DROIDSDK_INSTALL}/dsdk.sh" ]] && source "${DROIDSDK_INSTALL}/dsdk-init.sh"
```

Replace ``${DROIDSDK_INSTALL}`` with the path to your installation.