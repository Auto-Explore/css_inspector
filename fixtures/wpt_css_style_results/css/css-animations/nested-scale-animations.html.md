# css/css-animations/nested-scale-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/nested-scale-animations.html"
}
```

## style[0]

```css

@keyframes scale {
  0% {transform: scale(1);}
  1% {transform: scale(10);}
  100% {transform: scale(10);}
}
.animate {
  animation: scale 1s forwards;
  position: relative;
  top: 40%;
  left: 40%;
  width: 20%;
  height: 20%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
