# css/css-overflow/line-clamp/block-ellipsis-019.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-019.html"
}
```

## style[0]

```css

.container {
  position: relative;
  font: 25px/25px Ahem;
}
.bg, .clamp {
  position: absolute;
  top: 0;
  left: 0;
}
.bg {
  height: 100px;
  width: 100px;
  color: red;
  background-color: green;
}
.clamp {
  max-width: 100px;
  line-clamp: 1;
  color: green;
}
.red, .clamp::first-letter {
  color: red;
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
