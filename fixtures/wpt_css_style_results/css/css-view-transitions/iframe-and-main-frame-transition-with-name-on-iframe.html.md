# css/css-view-transitions/iframe-and-main-frame-transition-with-name-on-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/iframe-and-main-frame-transition-with-name-on-iframe.html"
}
```

## style[0]

```css

iframe {
  position: fixed;
  top: 0;
  left: 0;
  width: 50vw;
  height: 50vh;
  view-transition-name: inner;
}

.old {
  border: 1px solid black;
}

.new {
  border: 1px solid orange;
}

/* The main frame is showing the old screenshot for the root */
::view-transition-group(root) {
  animation-duration: 300s;
}
::view-transition-new(root) {
  animation: unset;
  opacity: 0;
}
::view-transition-old(root) {
  animation: unset;
  opacity: 1;
}

/* The iframe is showing the live screenshot */
::view-transition-new(inner) {
  animation: unset;
  opacity: 1;
}
::view-transition-old(inner) {
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

## style[1]

```css

  /* The iframe document itself is showing an old screenshot */
  ::view-transition-group(root) {
    animation-duration: 300s;
  }
  ::view-transition-new(root) {
    animation: unset;
    opacity: 0;
  }
  ::view-transition-old(root) {
    animation: unset;
    opacity: 1;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
