# css/css-view-transitions/object-view-box-new-image.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/object-view-box-new-image.html"
}
```

## style[0]

```css

.target {
  contain: paint;
  width: 100px;
  height: 100px;
  transform: scale(2.0, 3.0);
  position: relative;
  top: 200px;
  left: 200px;
  view-transition-name: target;
}

.embedded {
  width: 100%;
  height: 50%;
}

.hidden {
  contain: paint;
  width: 10px;
  height: 10px;
  background: grey;
  view-transition-name: hidden;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

html::view-transition-old(target) {
  animation: unset;
  opacity: 0;
  height: 100%;
}
html::view-transition-new(target) {
  animation: unset;
  opacity: 1;
  object-view-box: inset(50px 0px 0px 0px);
  object-fit: none;
  object-position: 0% 0%;
  height: 100%;
  contain: paint;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
