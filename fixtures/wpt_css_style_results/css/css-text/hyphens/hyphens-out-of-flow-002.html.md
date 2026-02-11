# css/css-text/hyphens/hyphens-out-of-flow-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/hyphens/hyphens-out-of-flow-002.html"
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
  hyphens: auto;
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
