# css/css-shadow/scoped-reference-animation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/scoped-reference-animation-001.html"
}
```

## style[0]

```css

/* Overrides predefined lower-roman counter-style */
@counter-style lower-roman {
  system: cyclic;
  symbols: 'X';
}

/* Overrides predefined upper-roman counter-style */
@counter-style upper-roman {
  system: cyclic;
  symbols: 'O';
}

/* Should resolve to the custom counter styles, not the predefined ones */
@keyframes list-style-type-anim {
  from { list-style-type: lower-roman; }
  to { list-style-type: upper-roman; }
}

#target1 {
  animation: list-style-type-anim 2s -0.9s linear paused;
}

#target2 {
  animation: list-style-type-anim 2s -1s linear paused;
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
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
