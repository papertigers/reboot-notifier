# reboot-notifier

This is a small rust program that's responsible for sending a pushover
notification when the server first boots up.

`reboot-notifier` provides a transient SMF service that you can modify for your
 env like so:

```
$ pfexec svccfg import smf/reboot-notifier.xml
$ pfexec svccfg -s reboot-notifier setprop start/user = mike
$ pfexec svccfg -s reboot-notifier setprop start/group = mike
$ pfexec svccfg -s reboot-notifier setprop config/file = /etc/reboot-notifier.toml
$ pfexec svcadm refresh reboot-notifier
$ pfexec svcadm enable reboot-notifier
```

Check out [example-config.toml](example-config.toml) for all of the config
options needed to use `reboot-notifier`.
