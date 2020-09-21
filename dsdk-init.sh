# minimalistic wrapper shell script needed to set environment variables of the current shell
# (doing it from within Rust is practically impossible)

setvars_path="$HOME/.droidsdk/setvars.sh"

function dsdk() {
  rm "$setvars_path" > /dev/null 2>&1

  # TODO: don't hardcode cli exec path
  ./target/debug/droidsdk "$@" || {
    echo 'Failed invoking exec' ;
    return 1;
  }

  if [ -f "$setvars_path" ]; then
    echo "Sourcing from setvars.sh"
    # TODO don't suppress if possible
    # shellcheck disable=SC1090
    source "$setvars_path"
  fi
}