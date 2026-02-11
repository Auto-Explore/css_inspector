# css/css-text/white-space/text-wrap-balance-004.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-balance-004.html"
}
```

## style[0]

```css

section {
    width: 50ch;
    font-family: monospace;
}
.test, .ref {
    text-wrap: balance;
}
.test { color: blue; }
.ref { color: orange; }
.mis { color: magenta; }

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
