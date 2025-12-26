default:
  # cargo check
  nickel export config/init.ncl > config/init.json
  watchexec -r -e rs,toml -- just r

# start dev window
r:
  nixGLMesa cargo run --features iced/time-travel

# release build
b:
  cargo build --release

# re-eval config file
c:
  nickel export config/init.ncl > config/init.json
  # bat config/init.json

# reload environment
d:
  direnv reload
