# css/css-view-transitions/navigation/resources/root-element-transition-iframe.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/resources/root-element-transition-iframe.html"
}
```

## style[0]

```css

@view-transition { navigation: auto; }

.hidden {
  width: 10px;
  height: 10px;
  view-transition-name: hidden;
  background: green;
}
html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { opacity: 0; }

html::view-transition-old(root) {
  width: 50vw;
  height: 100vh;
  position: fixed;
  left: 0px;
  top: 0px;
  opacity: 1;
  animation: unset;
}
html::view-transition-new(root) {
  width: 50vw;
  height: 100vh;
  position: fixed;
  left: 50vw;
  top: 0px;
  opacity: 1;
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
