cargo build --release
sudo systemctl stop monitor_switcher
mv target/release/monitor_switcher ~/MonitorSwitcher/
cp Switcher.toml ~/MonitorSwitcher/
sudo systemctl start monitor_switcher