# css/css-view-transitions/snapshot-containing-block-includes-scrollbar-gutter.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/snapshot-containing-block-includes-scrollbar-gutter.html"
}
```

## style[0]

```css

  :root {
    scrollbar-gutter: stable both-edges;
    /* unset so ::view-transition is visible. */
    view-transition-name: none;
  }
  ::view-transition {
    background-color: palegreen;
  }
  #target {
    position: absolute;
    top: 100px;
    left: 0px;
    width: 200px;
    height: 200px;
    background-color: limegreen;
    view-transition-name: target;
  }
  ::view-transition-group(target) {
    animation-duration: 300s;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
