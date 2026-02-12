# css/css-view-transitions/content-with-overflow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-overflow-ref.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  contain: paint;
  background: blue;
  overflow-clip-margin: 50px;
  view-transition-name: target;
}
.child {
  width: 200px;
  height: 200px;
  position: relative;
  top: 50px;
  left: 50px;
  background: green;
}
body { background: lightpink; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
