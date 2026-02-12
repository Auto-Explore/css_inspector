# css/css-overflow/line-clamp/block-ellipsis-020.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-020.html"
}
```

## style[0]

```css

.container {
  position: relative;
  font: 25px/25px Ahem;
}
.bg, .parent {
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
.parent::first-line, .red {
  color: red;
}
.clamp {
  max-width: 100px;
  line-clamp: 1;
  color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
