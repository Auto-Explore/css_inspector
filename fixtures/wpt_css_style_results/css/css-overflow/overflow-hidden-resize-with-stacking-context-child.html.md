# css/css-overflow/overflow-hidden-resize-with-stacking-context-child.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-hidden-resize-with-stacking-context-child.html"
}
```

## style[0]

```css

#container {
  overflow: hidden;
  width: 100px;
  height: 20px;
}
#stacking-context {
  position: relative;
  background: red;
  z-index: 100;
}
.content {
  width: 100px;
  height: 20px;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
