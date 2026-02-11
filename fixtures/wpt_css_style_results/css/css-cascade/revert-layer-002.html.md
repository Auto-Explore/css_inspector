# css/css-cascade/revert-layer-002.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-002.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
}

@layer {
  #target { background-color: green; }
}

#target {
  background-color: red;
  background-color: revert-layer;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
