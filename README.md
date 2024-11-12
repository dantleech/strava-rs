Strava TUI
==========

Strava TUI written in Rust! This is an experimental TUI for Strava.

Features:

- List activities in a comparable way
- Filter activites by with expressions
- Sort listed activities
- Display the route
- Show laps
- Race predictions
- Filter by route similarity ("anchoring")

## Screenshots

### List activities

![image](https://github.com/user-attachments/assets/f13ed611-d764-4941-a3df-c95db8636ba7)

### Acivity View

![image](https://github.com/user-attachments/assets/88c9b34a-7cee-409d-9d01-39bd22ef8259)

## Key Map

- `q`: **Quit**: quit!
- `k`: **Up** - select previous activity
- `j`: **Down** - select next activity
- `K`: **PageUp** - select previous activity
- `J`: **PageDown** - select next activity
- `n`: **Next** - (in activity view) next split
- `p`: **Previous** - (in activity view) previous split
- `o`: **ToggleSortOrder** - switch between ascending and descending order
- `u`: **ToggleUnitSystem** - switch between imperial and metric units
- `s`: **Sort** - show sort dialog
- `S`: **Rank** - choose ranking
- `f`: **Filter** - filter (see filter section below)
- `r`: **Refresh** - reload activities
- `a`: **Anchor** - show activities with similar routes
- `+`: **IncreaseTolerance** - incease the anchor tolerance
- `-`: **DecreaseTolerance** - descrease the ancor tolerance
- `0`: **ToggleLogView** - toggle log view

## Filter

Press `f` on the activity list view to open the filter input.

### Examples

Show all runs that are of a half marathon distance or more:

```
type = "Run" and distance > 21000
```

Show all runs with "Park" in the title:

```
type = "Run" and title ~ "Park"
```

### Fields

- `distance`: Distance (in meters)
- `type`: `Run`, `Ride` etc.
- `heartrate`: Heart rate in BPM.
- `title`: Activity title
- `elevation`: Elevation (in meters)
- `time`: Time (in seconds, 3600 = 1 hour)
- `date`: Date (YYYY-MM-DD)
- `speed`: Speed (meters per hour, 1000 = 1kmph)

### Values

- `kmph`: Kilometers per hour e.g. `speed > 10kmph`
- `mph`: Miles per hour e.g. `speed > 10mph`

### Operators

- `>`, `<`: Greater than, Less than (e.g. `distance > 21000`)
- `and`, `or`: Logical operators (e.g. `type = "Run" and time > 0`)
- `=`: Equal to
- `~`: String contains
- `!=`: Not equal to (e.g. `type != "Run"`)
- `!~`: String does not contain (e.g. `title ~ "Parkrun"`)
