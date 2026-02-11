# css/css-multicol/crashtests/float-multicol-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/crashtests/float-multicol-crash.html"
}
```

## style[0]

```css

#htmlvar00001 {
  shape-outside: inset(-1px 8px 1px 1px);
}

* {
  font-size: 1px;
  float: right;
  column-width: 7px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
