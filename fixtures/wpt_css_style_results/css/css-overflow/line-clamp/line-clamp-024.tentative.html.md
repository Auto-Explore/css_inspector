# css/css-overflow/line-clamp/line-clamp-024.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-024.tentative.html"
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
  background-color: purple;
  margin: 4px;
  /* Having a border means the margins of the .inner boxes won't collapse */
  border: 1px solid black;
}
.inner .inner {
  background-color: orange;
  white-space: pre;
}
.clamped-margin {
  /* These boxes are after the clamp point, so their margins should not affect
   * the size of their parent boxes. */
  background-color: red;
  margin: 20px;
  height: 20px;
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
