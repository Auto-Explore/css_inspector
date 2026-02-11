# css/css-color/oklch-009.html

```json
{
  "format_version": 3,
  "file": "css/css-color/oklch-009.html"
}
```

## style[0]

```css

    body { background-color: grey; }
    .test { background-color: hsl(0, 100%, 50%); width: 12em; height: 12em; }
    .test { background-color: oklch(100% 110 60); } /* l = 100% should always be white */
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
