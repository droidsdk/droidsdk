# droidsdk

DroidSDK, alternatively ``dsdk``, is a command line utility similar to SDKMAN! (in fact, **very** similar
to SDKMAN!), but written not via unportable bash shell scripts, but via in an actual, proper, 
maintainable programming language: Rust.

# Compilation

As of now, you *will* need to compile from source. Thankfully, Rust is pretty strict and locked-in with its
installation, so it's unlikely you'll have environment specific compilation issues. Installing Rust is 
very easy on Linux and only a tiny bit harder on Windows.

``cargo build`` in the root of the project builds everything.

# Installation

The [automatic installation](docs/installation/automatic.md) probably will work.

If it doesn't, or if it doesn't cover your specific linux setup, or if you just want to know exactly what you're doing,
follow the most applicable guide in [this folder](docs/installation). Installing dsdk is pretty easy and doesn't involve
many steps.

# Usage

[Usage.](docs/usage/Basics.md)