# css/filter-effects/backdrop-filter-plus-filter.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-plus-filter.html"
}
```

## style[0]

```css

div {
  position: absolute;
  width: 100px;
  height: 100px;
}
.container {
  width:200px;
  height:200px;
  top: 50px;
  position:absolute;
}
.blur {
  filter: invert(1);
}
.blur-bd {
  backdrop-filter: blur(10px);
}
.orangebox {
  left: 10px;
  top: 50px;
  background: green;
}
.bluebox {
  left: 60px;
  top: 110px;
  background: #00ff0055;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
