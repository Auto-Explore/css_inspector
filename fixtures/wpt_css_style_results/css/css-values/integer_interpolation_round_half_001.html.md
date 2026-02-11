# css/css-values/integer_interpolation_round_half_001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/integer_interpolation_round_half_001.html"
}
```

## style[0]

```css


#flex-container {
    display: flex;
    animation: anim-order 4s steps(4) forwards 1;
    animation-delay: -1s;
    animation-play-state: paused;
}

@keyframes anim-order {
  from {
    order: -2;
  }

  to {
    order: 0;
  }
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
