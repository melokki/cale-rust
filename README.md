# Cale

`Cale` it's a simple cli application to manage events. All the events are stored into a `sqlite3` database.

## Requirements

In order to use `Cale` you need to have to following packages installed:

- rust
- sqlite3

## How to use it

1. Clone the repository
2. Run `cargo build --release`
3. Run `./target/release/cale --help` to see the available commands

## Commands

1. `add` - add a new event to the database

Ex:

```bash
cale add 'event 1' '2023-10-17 08:00' '2023-10-17 09:00'
```

Ex: (force event creation)

```bash
cale add 'event 1' '2023-10-17 08:00' '2023-10-17 09:00' --allow-overlap
```

By default, if the event overlaps with another event, it will not be added to the database. To force the insertion of the event, use the `--allow-overlap` flag.

2. `list` - list all the events in the database within the given range

Ex:

```bash
cale list <start_date> <end_date>
```

3. `delete` - delete an event from the database

```bash
cale delete <event_id>
```

4. `update` - Update an event in the database. Just like the `add` command, `update` command, by default, will not allow the update of an event if it overlaps with another event. To force the update of the event, use the `--allow-overlap` flag.

Ex:

```bash
cale update 'event 1' '2023-10-17 08:00' '2023-10-17 09:00'
```

Ex: (force event update)

```bash
cale update 'event 1' '2023-10-17 08:00' '2023-10-17 09:00' --allow-overlap
```

5. `show` - show an event from the database

```bash
cale show <event_id>
```
