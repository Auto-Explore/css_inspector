# css/css-gaps/animation/rule-color-interpolation-conversion-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-color-interpolation-conversion-002.html"
}
```

## style[0]

```css

    #target {
      column-rule-color: blue, blue;
      animation: color-anim 2s linear paused;
    }
    @keyframes color-anim {
      from { column-rule-color: red, repeat(auto, red); }
    }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “column-rule-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
