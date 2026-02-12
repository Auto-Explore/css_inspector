# css/css-view-transitions/new-content-changes-overflow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-changes-overflow-ref.html"
}
```

## style[0]

```css

body { background: pink }
#target {
  position: relative;
  background: green;
  width: 100px;
  height: 100px;
  view-transition-name: target;
}
#child {
  background: blue;
  position: relative;
  top: 20px;
  left: 30px;
  width: 50px;
  height: 100px;
}
#child.large {
  height: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
