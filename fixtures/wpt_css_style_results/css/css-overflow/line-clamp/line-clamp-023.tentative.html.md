# css/css-overflow/line-clamp/line-clamp-023.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-023.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 4px;
  background-color: yellow;
  border: 2px solid black;
}
.inner {
  background-color: orange;
  margin: 4px;
  /* There is no border, so the margins of the .inner boxes will collapse */
}
.inner .inner {
  white-space: pre;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
