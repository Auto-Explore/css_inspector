# css/css-transforms/animation/rotate-animation-on-svg.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/rotate-animation-on-svg.html"
}
```

## style[0]

```css


@keyframes rotate-animation {
    from { rotate: 0; }
    to   { rotate: 180deg; }
}

svg {
    width: 400px;
    height: 400px;
    overflow: visible;
}

rect {
    width: 100px;
    height: 100px;
    transform-origin: 100px 100px;
    animation: rotate-animation 1ms linear forwards;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
