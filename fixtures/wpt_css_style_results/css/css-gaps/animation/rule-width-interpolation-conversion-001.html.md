# css/css-gaps/animation/rule-width-interpolation-conversion-001.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-width-interpolation-conversion-001.html"
}
```

## style[0]

```css

    #target {
      column-rule-width: 10px;
      column-rule-style: solid;
      animation: width-anim 2s linear paused;
    }
    @keyframes width-anim {
      from { column-rule-width: 0px; }
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
