# css/css-gaps/animation/rule-color-interpolation-conversion-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-color-interpolation-conversion-001.html"
}
```

## style[0]

```css

    #target {
      column-rule-color: blue;
      animation: color-anim 2s linear paused;
    }
    @keyframes color-anim {
      from { column-rule-color: red; }
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
