# css/compositing/isolation/isolation-establishes-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/compositing/isolation/isolation-establishes-stacking-context.html"
}
```

## style[0]

```css

#parent {
  width: 100px;
  height: 100px;
  isolation: isolate;
  background: red;
}
#child {
  width: 100px;
  height: 100px;
  position: relative;
  z-index: -1;
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
