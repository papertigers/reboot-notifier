# reboot-notifier

This is a small rust program that's responsible for sending a pushover
notification when the server first boots up.

`reboot-notifier` provides a transient SMF service that you can modify for your
 env like so:

```
$ pfexec svccfg import smf/reboot-notifier.xml
$ pfexec svccfg -s reboot-notifier setprop config/message = \"Server rebooted...\"
$ pfexec svccfg -s reboot-notifier setprop config/user_token = real_user_token
$ pfexec svccfg -s reboot-notifier setprop config/application_token = real_app_token
$ pfexec svcadm refresh reboot-notifier
$ pfexec svcadm enable reboot-notifier
```

Note that the provided SMF service is using the `read_authorization` propval to
hide sensitive configuration values. Refer to `smf_security(7)` for more
information.
