# Install DSDK on Windows

Windows command line sucks.

Nevertheless, DSDK supports the command line, albeit the support is rather flaky and requires much more manual
and tedious setup compared to a unix system.

Note that these instructions are merely a guideline, and your specific setup may (and in some cases should) differ.

## Install Rust

You will need Rust to compile DSDK. This is temporary. In future, binaries will be distributed for all major systems.

## Download and install DSDK

Clone the github repository into the folder you wish to use as your installation dir. Execute ``cargo build`` in that
directory.

## Unfortunately, CMD does not have a notion of a startup script.

Not by default, at least. Thankfully it is possible to enable a startup script by editing the Windows Registry.
Follow the instructions at https://superuser.com/a/144348

Then, in your newly-made CMD startup script, add this line:
``doskey dsdk=%DROIDSDK_PATH%\dsdk.bat $*`` where ``%DROIDSDK%`` is the path to DSDK's installation directory.