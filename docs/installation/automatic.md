# Automatic installation

``dsdk`` can now self-install via a single command invocation.

# Step 1 for all systems

~~Download the binaries~~ there are no binaries. There's only pain. And Rust. Which are kind of synonymous.

Install Rust. Compile the sources via ``cargo build`` in project's root.

# Step 2, Windows

Open an **admin cmd** shell (we need the elevated permissions to write to registry), then run ``dsdk setup`` in
the project's root.

CMD does not have a startup script by default, so this command does just that - creates a hacky startup script and
links it via the registry. See the manual install instructions if you're curious about the details. If the script
already exists, then you probably know better than us, and should follow the manual installation process.

# Step 2, Linux w/ bash

Only bash shell is currently supported (you can always install manually for your favourite shell, we just didn't 
automate it for you). 

``dsdk setup`` will simply add a ``source $dsdk_dir/dsdk_init.sh`` line at the end of your ``.bashrc`` file. 
Surprisingly, this doesn't require elevated permissions.