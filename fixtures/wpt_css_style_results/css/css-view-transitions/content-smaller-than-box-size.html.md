# css/css-view-transitions/content-smaller-than-box-size.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-smaller-than-box-size.html"
}
```

## style[0]

```css

div { contain: paint; }
#target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
#inner {
  width: 10px;
  height: 10px;
  background: blue;
  position: relative;
  top: 10px;
  left: 10px;
}

.hidden {
  background: pink;
  width: 10px;
  height: 10px;
  view-transition-name: hidden;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

html::view-transition-new(target) { animation: unset; opacity: 0; }
html::view-transition-old(target) { animation: unset; opacity: 1; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
