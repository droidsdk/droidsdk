# Basic DSDK usage

If you've used SDKMAN!, you probably already know how to use this tool. If you haven't,
read this page to learn the most important commands and features.

## Install some SDKits

The first thing you'd want to do after setting up DSDK on your system is to download the few
SDKits that immediately interest you.

If you want to know what SDKits are available, use the ``dsdk list`` command, without any arguments.
To find out what any of the returned names mean, use ``dsdk whatis [SDKit]``, e.g. ``dsdk whatis kotlin``. 

Each SDKit comes in many versions. Most candidates have a linear version history, however Java
gets a special treatment here because it has so many implementations. To find out which versions
are available, run ``dsdk list [SDKit]``, e.g. ``dsdk list java``. 
The item in square brackets is each version's unique identifier.

Then, to install the desired version, run ``dsdk install [SDKit] [version identifier]``, 
e.g. ``dsdk install java 7.0.262-zulu``. This will, obviously, require internet connectivity.

## Use an SDKit

To actually do anything useful with ``dsdk``, you need to ``use`` the SDKit you just installed.
Doing it is rather straightforward indeed - run ``dsdk use [SDKit] [version identifier]``, 
e.g. ``dsdk use java 7.0.262-zulu``

This will set up your PATH environment so that whenever you invoke the specified SDKit, you'll end up
invoking the specified version that was downloaded and installed by ``dsdk``. Note that this applies only
to the shell in which you invoked this command. Other shells will not be affected.

Unlike in SDKMAN!, currently there is no way to change a candidate globally, for all shells. That will be implemented
in future versions.

If you, for whatever reason, need to go back to using the non-``dsdk`` SDKit you were using before, you
can run ``dsdk revert [SDKit]``