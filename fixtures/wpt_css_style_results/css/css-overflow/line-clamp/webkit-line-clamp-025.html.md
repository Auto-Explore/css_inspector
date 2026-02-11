# css/css-overflow/line-clamp/webkit-line-clamp-025.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-025.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 1;
  font: 16px / 32px monospace;
  white-space: pre;
  padding: 0 4px;
  background-color: yellow;
  width: 11.1ch;
  overflow: hidden; /* can be removed once implementations update their old -webkit-line-clamp implementations */
}
.float {
  float: right;
  color: orange;
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
