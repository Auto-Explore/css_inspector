# css/css-view-transitions/fractional-translation-from-transform.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fractional-translation-from-transform.html"
}
```

## style[0]

```css

body {
  width: 100vw;
  height: 100vh;
  background: grey;
}

#composited {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 0px;
  left: 0px;
  transform: translate(100.52px, 100.37px);
  will-change: transform;

  view-transition-name: composited;
}

#noncomposited {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 0px;
  left: 0px;
  transform: translate(100.52px, 250.37px);

  view-transition-name: noncomposited;
}

::view-transition-new(root), ::view-transition-old(root) {
  animation-play-state: paused;
}

::view-transition-new(composited),
::view-transition-new(noncomposited) {
  opacity: 1;
  animation: unset;
}
::view-transition-old(composited),
::view-transition-old(noncomposited) {
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
