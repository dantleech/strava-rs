Strava TUI
==========

Implementation
--------------

- Tokyo / async
- Offline file based database (JSON)
- TUI and keyboard driven

Key Map
-------

- `n`: Next tab
- `p`: Previous tab
- `j`: Down
- `p`: Up
- ``: Return to activities view
- `⏎`: Select

Features
--------

- Sync with Strava
- Automatically group similar routes as "attempts"
- **Switch between KM and miles easily**
- Modes:
  - Activities: list and search activities
  - Activity: display activity (with tabs for pages)

Activities
----------

### List

- Date
- Title
- Distance
- Time
- Avg Pace  ::    Additional: Race "predictions"
- Avg heart rate  ::    Additional: Race "predictions"


```
+-----------------------------------------------+
| Activities                                    |
+-----------------------------------------------+
| List                                          |
+-----------------------------------------------+
|  Date     Attempt Title   Dist   Time  Pace   |
|  ----     ------- -----   ----   ----  ----   |
|  Mon 16th 21      Parkrun 3.1m   20:00 7:02   |
|> Sun 17th 2       Half    13.2m  20:00 7:02 < |
|                                               |
+-----------------------------------------------+
| [k] Up [j] Down [⏎] Select |    [u] km/m |
```

Selecting an entry takes you to the acitvity view.

Activity
--------

### Summary
  
```
+-----------------------------------------------+
| Park Run Esplanade                            |
+-----------------------------------------------+
| Summary | Laps | Attempts                     |
+-----------------------------------------------+
|                                               |
| Time:      20:00  Distance:   3.1km           |
| Avg. Pace: 7:04   Avg. Heart: 160bpm          |
|                                               |
+-----------------------------------------------+
| [k] Up [j] Down [⏎] Select |    [u] km/m |
```
