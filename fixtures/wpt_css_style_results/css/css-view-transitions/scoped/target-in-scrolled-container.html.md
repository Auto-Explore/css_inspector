# css/css-view-transitions/scoped/target-in-scrolled-container.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/target-in-scrolled-container.html"
}
```

## style[0]

```css

#scroller {
  width: 200px;
  height: 200px;
  overflow: scroll;
  background: lightgray;
}
#target {
  width: 50px;
  height: 50px;
  background: green;
  margin-top: 100px;
  view-transition-name: target;
}
#spacer {
  height: 200px;
}

html::view-transition-group(*) { animation-play-state: paused; }
html::view-transition-old(*) { animation: none; opacity: 1; }
html::view-transition-new(*) { animation: none; opacity: 0; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
