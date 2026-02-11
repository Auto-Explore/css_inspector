# css/css-writing-modes/crashtests/orthogonal-percent-height-multicol-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/crashtests/orthogonal-percent-height-multicol-crash.html"
}
```

## style[0]

```css

.CLASS2 {
  writing-mode: vertical-lr;
}
.CLASS4 {
  columns: +6.9vh 8589934576;
}
samp, button, div.CLASS1 {
  overflow-x: auto;
  float: left;
  block-size: 2.0%;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
