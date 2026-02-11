# css/css-text/hyphens/hyphens-out-of-flow-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-out-of-flow-001.html"
}
```

## style[0]

```css

span {
  position: absolute;
  color: transparent;
}
div {
  border: solid orange;
  margin: 5px;
  width: 6ch;
  hyphens: manual;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
