# css/css-overflow/line-clamp/line-clamp-007.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-007.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 3;
  font: 16px / 32px serif;
  white-space: pre;
  padding: 0 4px;
  background-color: yellow;
}
.child {
  overflow: auto;
  font: 24px / 48px serif;
  color: blue;
  padding: 0 4px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
