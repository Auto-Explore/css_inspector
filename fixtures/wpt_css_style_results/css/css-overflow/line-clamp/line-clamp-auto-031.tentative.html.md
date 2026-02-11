# css/css-overflow/line-clamp/line-clamp-auto-031.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-031.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: calc(4lh + 2 * 5px);
  font: 16px / 32px serif;
  background-color: orange;
}
.inner {
  white-space: pre;
  margin: 5px;
  background-color: yellow;
}
.collapse-through {
  margin: 10px;
}
.rel {
  position: relative;
}
.abspos {
  position: absolute;
  right: 0;
  height: 100px;
  width: 100px;
  background-color: skyblue;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
