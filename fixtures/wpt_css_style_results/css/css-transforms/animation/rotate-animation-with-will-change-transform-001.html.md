# css/css-transforms/animation/rotate-animation-with-will-change-transform-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/rotate-animation-with-will-change-transform-001.html"
}
```

## style[0]

```css


@keyframes a {
  from { rotate: 0 1 0 44deg; }
  to { rotate: 0 1 0 44deg; }
}

div {
  width: 100px;
  height: 100px;
  animation: a linear 10s infinite;
  /* rotate: 0 1 0 44deg; */
  background: fuchsia;
  transform-origin: 100px 0;
  will-change: transform;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
