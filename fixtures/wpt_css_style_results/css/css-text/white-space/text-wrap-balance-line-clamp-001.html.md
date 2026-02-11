# css/css-text/white-space/text-wrap-balance-line-clamp-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-balance-line-clamp-001.html"
}
```

## style[0]

```css

.container {
  font-family: monospace;
  font-size: 20px;
  width: 20ch;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
  text-wrap: balance;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
