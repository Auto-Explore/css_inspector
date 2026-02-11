# css/css-viewport/zoom/zoom-pseudo-image.html

```json
{
  "format_version": 3,
  "file": "css/css-viewport/zoom/zoom-pseudo-image.html"
}
```

## style[0]

```css

.icon {
  width: 200px;
  height: 200px;
  background-color: blue;
  margin-right: 5px;
  display: inline-block;
  vertical-align: top;
}

.icon::before {
  display: block;
  content: url(/images/green.png);
  width: 100px;
  height: 100px;
  background-color: purple;
}

.zoom {
  zoom: 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
