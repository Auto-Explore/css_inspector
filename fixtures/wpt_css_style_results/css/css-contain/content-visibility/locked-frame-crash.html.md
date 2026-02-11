# css/css-contain/content-visibility/locked-frame-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/locked-frame-crash.html"
}
```

## style[0]

```css

:first-child { content-visibility: hidden; }
html,body,ol { -webkit-column-count: 2; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “-webkit-column-count”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
