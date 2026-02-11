# css/css-overflow/line-clamp/webkit-line-clamp-036.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-036.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  width: 10.1ch;
  font: 16px / 32px monospace;
  background-color: yellow;
  padding: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
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
