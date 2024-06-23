# what-the-time

A command line tool `wtt` that parses JSON lines containing a datetime field (defaulting to "time") and calculates the time difference between each log line and the total clock time logged since the first line.

The calculations can be inserted as new JSON lines between the original lines or as new JSON fields.

## Installation (macOS)

You can install directly from my homebrew tap:

```
brew install sdball/tap/what-the-time
```

Or you can configure homebrew to have my tap loaded

```
brew tap sdball/tap
brew install what-the-time
```

## Usage

- `-i`: insert the milliseconds since the previous line in a `millis_since_previous_line` field in a new JSON line inserted between each original line
- `-s`: insert the milliseconds since the first line in a `millis_since_start` field in a new JSON line inserted between each original line
- `-I`: inject the milliseconds since the previous line as a `millis_since_previous_line` field into each line following the first
- `-S`: inject the milliseconds since the first line as a `millis_since_start` field into each line following the first

For example say we start from this simple file of ten JSON lines

`sample_data/ten-lines.json.log`

```json
{"time": "2024-06-23T00:00:00Z", "event": "start"}
{"time": "2024-06-23T00:00:01Z", "event": "processing"}
{"time": "2024-06-23T00:00:02Z", "event": "processing"}
{"time": "2024-06-23T00:00:03Z", "event": "processing"}
{"time": "2024-06-23T00:00:04Z", "event": "processing"}
{"time": "2024-06-23T00:00:05Z", "event": "processing"}
{"time": "2024-06-23T00:00:06Z", "event": "processing"}
{"time": "2024-06-23T00:00:07Z", "event": "processing"}
{"time": "2024-06-23T00:00:08Z", "event": "processing"}
{"time": "2024-06-23T00:00:09Z", "event": "end"}
```

Each "time" field is 1000ms apart and a total of 9000ms is presented since the first line. We can find that data using `wtt` in various ways.

### insert millis since previous line

```
$ cat sample_data/ten-lines.json.log | wtt -i
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:01Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:02Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:03Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:04Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:05Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:06Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:07Z"}
{"millis_since_previous_line":1000}
{"event":"processing","time":"2024-06-23T00:00:08Z"}
{"millis_since_previous_line":1000}
{"event":"end","time":"2024-06-23T00:00:09Z"}
```

### insert total millis since first line

```
$ cat sample_data/ten-lines.json.log | wtt -s
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"millis_since_start":1000}
{"event":"processing","time":"2024-06-23T00:00:01Z"}
{"millis_since_start":2000}
{"event":"processing","time":"2024-06-23T00:00:02Z"}
{"millis_since_start":3000}
{"event":"processing","time":"2024-06-23T00:00:03Z"}
{"millis_since_start":4000}
{"event":"processing","time":"2024-06-23T00:00:04Z"}
{"millis_since_start":5000}
{"event":"processing","time":"2024-06-23T00:00:05Z"}
{"millis_since_start":6000}
{"event":"processing","time":"2024-06-23T00:00:06Z"}
{"millis_since_start":7000}
{"event":"processing","time":"2024-06-23T00:00:07Z"}
{"millis_since_start":8000}
{"event":"processing","time":"2024-06-23T00:00:08Z"}
{"millis_since_start":9000}
{"event":"end","time":"2024-06-23T00:00:09Z"}
```

### inject milliseconds since the previous line

```
$ cat sample_data/ten-lines.json.log | wtt -I
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:01Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:02Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:03Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:04Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:05Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:06Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:07Z"}
{"event":"processing","millis_since_previous_line":1000,"time":"2024-06-23T00:00:08Z"}
{"event":"end","millis_since_previous_line":1000,"time":"2024-06-23T00:00:09Z"}
```

### inject total milliseconds since the first line

```
$ cat sample_data/ten-lines.json.log | wtt -S
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"event":"processing","millis_since_start":1000,"time":"2024-06-23T00:00:01Z"}
{"event":"processing","millis_since_start":2000,"time":"2024-06-23T00:00:02Z"}
{"event":"processing","millis_since_start":3000,"time":"2024-06-23T00:00:03Z"}
{"event":"processing","millis_since_start":4000,"time":"2024-06-23T00:00:04Z"}
{"event":"processing","millis_since_start":5000,"time":"2024-06-23T00:00:05Z"}
{"event":"processing","millis_since_start":6000,"time":"2024-06-23T00:00:06Z"}
{"event":"processing","millis_since_start":7000,"time":"2024-06-23T00:00:07Z"}
{"event":"processing","millis_since_start":8000,"time":"2024-06-23T00:00:08Z"}
{"event":"end","millis_since_start":9000,"time":"2024-06-23T00:00:09Z"}
```

### multiple flags

You can, of course, supply multiple flags for the behavior you want.

```
❯ cat sample_data/ten-lines.json.log | wtt -is
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"millis_since_previous_line":1000,"millis_since_start":1000}
{"event":"processing","time":"2024-06-23T00:00:01Z"}
{"millis_since_previous_line":1000,"millis_since_start":2000}
{"event":"processing","time":"2024-06-23T00:00:02Z"}
{"millis_since_previous_line":1000,"millis_since_start":3000}
{"event":"processing","time":"2024-06-23T00:00:03Z"}
{"millis_since_previous_line":1000,"millis_since_start":4000}
{"event":"processing","time":"2024-06-23T00:00:04Z"}
{"millis_since_previous_line":1000,"millis_since_start":5000}
{"event":"processing","time":"2024-06-23T00:00:05Z"}
{"millis_since_previous_line":1000,"millis_since_start":6000}
{"event":"processing","time":"2024-06-23T00:00:06Z"}
{"millis_since_previous_line":1000,"millis_since_start":7000}
{"event":"processing","time":"2024-06-23T00:00:07Z"}
{"millis_since_previous_line":1000,"millis_since_start":8000}
{"event":"processing","time":"2024-06-23T00:00:08Z"}
{"millis_since_previous_line":1000,"millis_since_start":9000}
{"event":"end","time":"2024-06-23T00:00:09Z"}
```

## Tips

If you have some sparse events in your JSON log lines it's nice to filter down to those lines and then calculate the timing between them

```
$ cat sample_data/ten-lines.json.log | rg '(start|end)' | wtt -is
```

```json
{"event":"start","time":"2024-06-23T00:00:00Z"}
{"millis_since_previous_line":9000,"millis_since_start":9000}
{"event":"end","time":"2024-06-23T00:00:09Z"}
```
