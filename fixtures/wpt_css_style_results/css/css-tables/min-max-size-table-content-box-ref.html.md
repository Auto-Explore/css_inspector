# css/css-tables/min-max-size-table-content-box-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/min-max-size-table-content-box-ref.html"
}
```

## style[0]

```css

html,
body {
  color: black;
  background-color: white;
  font: 16px/0 monospace;
  padding: 0;
  margin: 0;
}

.table {
  box-sizing: content-box;
  border: 3px solid black;
  padding: 5px;
  background: blue content-box;
  display: inline-block;
}

.big {
  width: 75px;
  height: 75px;
}

.td {
  padding: 1px;
}

.grid {
  display: grid;
  width: 75px;
  height: 75px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
