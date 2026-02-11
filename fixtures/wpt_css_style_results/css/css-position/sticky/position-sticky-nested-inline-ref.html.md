# css/css-position/sticky/position-sticky-nested-inline-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-nested-inline-ref.html"
}
```

## style[0]

```css

.group {
  display: inline-block;
  position: relative;
  width: 150px;
  height: 250px;
}

.scroller {
  position: relative;
  width: 100px;
  height: 200px;
  overflow-x: hidden;
  overflow-y: auto;
  font: 25px/1 Ahem;
}

.contents {
  height: 500px;
}

.outerIndicator {
  color: green;
  position: absolute;
  left: 0;
}

.innerIndicator {
  color: yellow;
  position: absolute;
  left: 25px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
