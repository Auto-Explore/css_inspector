# css/css-view-transitions/content-with-inline-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-inline-child-ref.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  overflow-clip-margin: 500px;
  contain: paint;
  view-transition-name: target;
  background-color: grey;
}

#child {
  position: relative;
  left: 100px;
  top: 100px;
  color: lightgreen;
  background-color: darkgreen;
}

#innerchild {
  position: relative;
  left: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
