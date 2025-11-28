yoctojson
---------

A tiny (< 300 LOC) application made for parsing JSON input, and prettifying it, in the simplest way possible.

Supports reading from files as well as `stdin`.

```bash
yoctojson test_files/large-file.json
cat test_files/large-file.json | yoctojson
```