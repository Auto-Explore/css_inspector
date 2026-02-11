# css/css-overflow/line-clamp/line-clamp-with-fixed-pos-015.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-fixed-pos-015.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 4lh;
  font: 16px / 32px serif;
  padding: 0 4px;
  background-color: yellow;
}
.transformed {
  transform: scale(1); /* Makes it a fixed-pos containing block */
}
.fixed {
  position: fixed;
  width: 100px;
  height: 100px;
  background-color: skyblue;
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
