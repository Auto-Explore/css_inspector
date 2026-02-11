# css/css-overflow/line-clamp/line-clamp-with-text-overflow-string-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-text-overflow-string-002.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 4;
  line-clamp: 4;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
  padding: 0 4px;
  text-overflow: "-";
  overflow: hidden;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-box-orient”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
