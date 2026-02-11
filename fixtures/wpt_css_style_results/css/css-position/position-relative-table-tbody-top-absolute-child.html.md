# css/css-position/position-relative-table-tbody-top-absolute-child.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-relative-table-tbody-top-absolute-child.html"
}
```

## style[0]

```css

table {
  border-collapse:collapse;
}

td {
  padding: 0;
}

td > div {
  height: 50px;
  width: 50px;
}

.group {
  display: inline-block;
  position: relative;
  width: 150px;
  height: 200px;
}

.indicator {
  position: absolute;
  background-color: red;
  left: 0;
  top: 100px;
  height: 50px;
  width: 50px;
}

.relative {
  position: relative;
  top: 50px;
  background-color: green;
}

.absolute {
  position: absolute;
  top: 50px;
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
