# css/css-view-transitions/fractional-translation-from-position.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fractional-translation-from-position.html"
}
```

## style[0]

```css

body {
  width: 100vw;
  height: 100vh;
  background: grey;
  font: 12px/1 Ahem;
}

#target {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 100.52px;
  left: 100.37px;

  view-transition-name: target;
  contain: layout;
}

::view-transition-new(root), ::view-transition-old(root) {
  animation-play-state: paused;
}

::view-transition-new(target) {
  opacity: 1;
  animation: unset;
}
::view-transition-old(target) {
  opacity: 0;
  animation: unset;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
