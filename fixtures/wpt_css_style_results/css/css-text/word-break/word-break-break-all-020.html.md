# css/css-text/word-break/word-break-break-all-020.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-break/word-break-break-all-020.html"
}
```

## style[0]

```css

div {
  border: 5px solid;
  width: 2em;
}

div:nth-of-type(1) { border-color: blue; }
div:nth-of-type(2) { border-color: green; }
div:nth-of-type(3) { border-color: orange; }

div:nth-of-type(3) { word-break: break-all; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
