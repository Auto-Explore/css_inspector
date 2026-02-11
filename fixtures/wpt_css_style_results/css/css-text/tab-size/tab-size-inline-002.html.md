# css/css-text/tab-size/tab-size-inline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/tab-size/tab-size-inline-002.html"
}
```

## style[0]

```css

div {
    font-family: monospace;
    white-space: pre;
    tab-size: 10;
    font-size: 2em;
}
span {
    tab-size: 4;
}
.wrong { color: orange; }
.right { color: blue; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
