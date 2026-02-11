# css/css-gaps/animation/rule-color-interpolation-conversion-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-color-interpolation-conversion-crash.html"
}
```

## style[0]

```css

  #target {
    animation: anim 1s paused;
  }

  @keyframes anim {
    from {
      column-rule-color: green;
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
