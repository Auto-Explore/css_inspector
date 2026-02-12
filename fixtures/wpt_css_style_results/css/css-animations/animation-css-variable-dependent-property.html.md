# css/css-animations/animation-css-variable-dependent-property.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-css-variable-dependent-property.html"
}
```

## style[0]

```css

@property --c {
  syntax: "<color>";
  inherits: true;
  initial-value: black;
}
@keyframes color-shift {
    0% {
        --c: black;
    }
    100% {
        --c: white;
    }
}
#target {
    color: var(--c);
    animation: color-shift 1s linear 1 forwards paused
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
