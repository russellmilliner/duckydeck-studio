#!/usr/bin/env sh
set -e

RULE_CONTENT='# duckyPad Pro & duckyPad (2020) udev rules
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11d", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11d", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11c", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11c", MODE="0660", TAG+="uaccess"
'

printf "%s" "$RULE_CONTENT" > /etc/udev/rules.d/70-duckypad.rules
chmod 644 /etc/udev/rules.d/70-duckypad.rules
udevadm control --reload-rules
udevadm trigger --subsystem-match=hidraw --attr-match=idVendor=0483
