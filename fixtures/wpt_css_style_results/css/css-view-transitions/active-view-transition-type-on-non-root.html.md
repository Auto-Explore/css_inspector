# css/css-view-transitions/active-view-transition-type-on-non-root.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/active-view-transition-type-on-non-root.html"
}
```

## style[0]

```css


body { background: lightpink; }

#target {
  view-transition-name: target;
  background: green;
  width: 100px;
  height: 100px;
}

main:active-view-transition-type(type-name) #target {
  background: red;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
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
