# css/css-view-transitions/scoped/scrolled-target-position.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/scrolled-target-position.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  overflow: scroll;
  background: lightblue;
}
#child {
  width: 50px;
  height: 75px;
  background: green;
}
#content {
  width: 100px;
  height: 400px;
  background: yellow;
}

::view-transition-group(*),
::view-transition-image-pair(*),
::view-transition-old(*),
::view-transition-new(*) {
  /* freeze all animations at start */
  animation-duration: 100000s;
  animation-timing-function: steps(1, jump-end);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
