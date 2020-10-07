# Install DSDK on other systems

While we offer no guarantees DSDK will work on systems other than the ones we explicitly support,
you may of course try at your own risk. Assuming the following facts:

 - DSDK binaries compile and execute successfully
 - your system has a scripting shell of sorts
 - the shell in question can execute script files
 - the shell allows said script files to change its environment variables (one way or another)
 
...DSDK probably will work on such a system.

## Clone & compile

The usual. Install Rust (if you don't have it already), clone the repository, run ``cargo build``. If it
doesn't fail, you're probably fine.

## Write a wrapper shell script for your shell

See ``dsdk-init.sh`` for a Linux/bash wrapper script example and ``dsdk.bat`` for a Windows/CMD example.
Or, alternatively, see the following pseudocode:

```
delete_file ( %DROIDSDK_HOME%/setvars_file_name )

let call_arguments = shell_script_invocation_args[1..]
run_command ( %DROIDSDK_INSTALLATION_DIR%/target/debug/droidsdk, call_arguments )

set_variables_from_file ( %DROIDSDK_HOME%/setvars_file_name )
```

## Optionally: add a shortcut to said wrapper script in your shell's startup script

This isn't strictly necessary (you can do this manually, obviously) but it is very nice to have.
On Linux this involves adding a couple of lines to ``~/.bashrc``, on Windows the process is a bit more involved
and requires changing the Registry. Check your shell's documentation to see if it supports a startup script
(most likely - yes) and how to change said script.