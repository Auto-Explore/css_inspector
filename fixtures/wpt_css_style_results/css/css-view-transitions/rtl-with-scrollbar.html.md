# css/css-view-transitions/rtl-with-scrollbar.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/rtl-with-scrollbar.html"
}
```

## style[0]

```css

#target {
  position: absolute;
  top: 100px;
  left: 100px;
  width: 100px;
  height: 200px;
  background: dodgerblue;
  contain: paint;
  view-transition-name: target;
}

#inroot {
  position: absolute;
  top: 300px;
  left: 200px;
  width: 100px;
  height: 200px;
  background: rebeccapurple;
  contain: paint;
}

body {
  margin: 0px;
  padding: 0px;
  /* add overflow for scrollbar */
  height: 200vh;
}

/* Show the old snapshot for 300s */
html::view-transition-group(*) {
  animation-duration: 300s;
  opacity: 1;
}
html::view-transition-new(*) {
  animation: unset;
  opacity: 0;
}
html::view-transition-old(*) {
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
