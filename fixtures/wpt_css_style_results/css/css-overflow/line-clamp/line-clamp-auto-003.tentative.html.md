# css/css-overflow/line-clamp/line-clamp-auto-003.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-003.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 138px; /* 32px * 4 + (1px + 4px) * 2 */
  box-sizing: border-box;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
  margin: 2px;
  border: 1px solid black;
  padding: 4px;
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
