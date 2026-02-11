# css/css-values/integer_interpolation_round_half_towards_positive_infinity_z_index.html

```json
{
  "format_version": 3,
  "file": "css/css-values/integer_interpolation_round_half_towards_positive_infinity_z_index.html"
}
```

## style[0]

```css


#anim-target {
  animation: anim-z 4s steps(4) forwards 1;
  animation-delay: -1s;
  animation-play-state: paused;
}

@keyframes anim-z {
  from {
    z-index: -2;
  }

  to {
    z-index: 0;
  }
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
