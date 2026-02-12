# css/css-view-transitions/no-named-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-named-elements.html"
}
```

## style[0]

```css

body { background: red; }

:root { view-transition-name: none; }

@keyframes no-op {
  from {opacity: 1;}
  to {opacity: 1;}
}

:root::view-transition {
  width: 100%;
  height: 100%;
  background: blue;
  animation: no-op 300s;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
