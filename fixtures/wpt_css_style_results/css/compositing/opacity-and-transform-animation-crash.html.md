# css/compositing/opacity-and-transform-animation-crash.html

```json
{
  "format_version": 3,
  "file": "css/compositing/opacity-and-transform-animation-crash.html"
}
```

## style[0]

```css

  @keyframes anim {
    from {
      opacity: 0.99;
    }
    to {
      opacity: 0.9;
      transform: scale(1, 1);
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
