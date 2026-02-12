# css/css-view-transitions/pseudo-with-classes-view-transition-image-pair.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-with-classes-view-transition-image-pair.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  position: absolute;
  background: green;
  view-transition-name: target;
  view-transition-class: cls;
}

::view-transition-group(*) {
  animation-play-state: paused;
}

::view-transition-image-pair(target.cls) {
  transform: translateX(100px);
  animation-play-state: paused;
}

::view-transition-old(*) {
  animation: unset;
  opacity: 1;
}

::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}


```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
