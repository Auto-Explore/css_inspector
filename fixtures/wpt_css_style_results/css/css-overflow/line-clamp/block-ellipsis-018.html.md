# css/css-overflow/line-clamp/block-ellipsis-018.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-018.html"
}
```

## style[0]

```css

.container {
  position: relative;
  height: 100px;
  width: 100px;
  font: 25px Ahem;
}
.bg {
  position: absolute;
  top: 0;
  left: 0;
  color: green;
  line-height: 25px;
}
.red {
  color: red;
}
.large {
  font-size: 1.5em;
}
.clamp {
  position: absolute;
  width: 100px;
  bottom: 0;
  left: 0;
  line-clamp: 1;

  color: transparent;
  background: linear-gradient(to top, green 0, green 1lh, red 1lh);
  background-position: left bottom;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
