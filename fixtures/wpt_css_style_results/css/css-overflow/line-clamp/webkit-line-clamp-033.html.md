# css/css-overflow/line-clamp/webkit-line-clamp-033.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-033.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
  padding: 0 4px;
  overflow: hidden; /* can be removed once implementations update their old -webkit-line-clamp implementations */
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
