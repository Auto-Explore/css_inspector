# css/css-ui/outline-auto-width-001.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-auto-width-001.html"
}
```

## style[0]

```css

div {
    outline-style: auto;
    outline-width: 1em;
    width: 100px;
    height: 100px;
    margin: 2em;
}
div + div {
    outline-width: 0;
}
hr ~ div {
    outline-style: solid;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
