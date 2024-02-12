# dpkgtable plugin for Nu shell

This a plugin for nushell that captures the output of dpkg --list into a table.

    ~> dpkgtable | where name == "git"
    ╭───┬────────┬──────┬──────────────┬──────────────┬─────────────────────────────────────────────────────╮
    │ # │ status │ name │   version    │ architecture │                     description                     │
    ├───┼────────┼──────┼──────────────┼──────────────┼─────────────────────────────────────────────────────┤
    │ 0 │ ii     │ git  │ 1:2.39.2-1.1 │ amd64        │ fast, scalable, distributed revision control system │
    ╰───┴────────┴──────┴──────────────┴──────────────┴─────────────────────────────────────────────────────╯

You can capture the oputput something like

    let packages = ( dpkgtable )

and then you can do something like

    $packages | where name =~ git

or

    dpkg --remove $packages.5.name

to remove the package #5 in the list (if running as root).
