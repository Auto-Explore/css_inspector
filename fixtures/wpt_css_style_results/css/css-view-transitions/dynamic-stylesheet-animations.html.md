# css/css-view-transitions/dynamic-stylesheet-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/dynamic-stylesheet-animations.html"
}
```

## style[0]

```css

:root { view-transition-name: none; }
#target {
    view-transition-name: target;
    background: red;
    width: 100px;
    height: 100px;
}
html::view-transition-group(*) {
    animation-timing-function: steps(2, start);
    animation-play-state: paused;
}
html::view-transition-old(*),
html::view-transition-new(*) {
    animation-play-state: paused;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
