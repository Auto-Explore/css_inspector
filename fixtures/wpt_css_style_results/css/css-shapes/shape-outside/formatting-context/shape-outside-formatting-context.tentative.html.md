# css/css-shapes/shape-outside/formatting-context/shape-outside-formatting-context.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/formatting-context/shape-outside-formatting-context.tentative.html"
}
```

## style[0]

```css

.test {
  width: 200px;
  overflow: hidden;
}

.float {
  float: left;
  width: 50%;
  height: 100px;

  background: orange;
  shape-outside: polygon(0 0, 100% 100%, 0 100%);
  clip-path: polygon(0 0, 100% 100%, 0 100%);
}

.flex {
  display: flex;
  height: 50px;
  background: rebeccapurple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
