# css/css-text/writing-system/writing-system-line-break-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/writing-system/writing-system-line-break-001.html"
}
```

## style[0]

```css

div {
  font-family: monospace;
  width: 2em;
  line-break: loose;
}
[lang=ja] { border: solid blue; }
[lang=ja-Hang] { border: solid orange; }
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
