# css/css-animations/crashtests/add-pseudo-while-animating-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/crashtests/add-pseudo-while-animating-001.html"
}
```

## style[0]

```css


@keyframes a {
    0% {
        border-radius: 1px;
    }
}

div {
    animation-name: a;
    animation-duration: 3s;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
