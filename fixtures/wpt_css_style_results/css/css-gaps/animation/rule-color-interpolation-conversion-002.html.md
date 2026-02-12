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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
