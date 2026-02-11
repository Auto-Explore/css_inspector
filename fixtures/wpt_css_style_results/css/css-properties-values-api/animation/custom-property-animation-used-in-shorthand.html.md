# css/css-properties-values-api/animation/custom-property-animation-used-in-shorthand.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/animation/custom-property-animation-used-in-shorthand.html"
}
```

## style[0]

```css


@property --angle {
  syntax: "<angle>";
  inherits: false;
  initial-value: 0deg;
}

@keyframes angle-animation {
  from { --angle: 0deg }
  to { --angle: 180deg }
}

#target {
  animation: angle-animation 1000s linear -500s paused;
  background: linear-gradient(var(--angle), black, white);
}

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
