# css/css-sizing/stretch/stretch-table-001.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/stretch/stretch-table-001.html"
}
```

## style[0]

```css

table {
  border-spacing: 0;
  margin-bottom: 3px;
}
td {
  border: 0;
  padding: 0;
}
.stretch {
    height: -webkit-fill-available;
    height: stretch;
}
.spacer {
  height: 10px;
  width: 20px;
  background: tan;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
