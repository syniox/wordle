VERSION=v0.16.0
curl https://sh.rustup.rs -sSf | sh -s -- -y
wget -qO- https://github.com/thedodd/trunk/releases/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk build --release
