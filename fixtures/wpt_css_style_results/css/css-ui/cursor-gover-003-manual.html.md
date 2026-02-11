# css/css-ui/cursor-gover-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-gover-003-manual.html"
}
```

## style[0]

```css

#d1 {
  width: 0;
  height: 0;
  position: relative;
  cursor: url("support/cursors/fail.png"), help;
}
#d2 {
  position: absolute;
  top: 0; left: 0;
  width: 100px;
  height: 100px;
  border: solid blue;
}
#d1:hover {
  cursor: crosshair;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
