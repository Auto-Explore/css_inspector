# css/css-gaps/animation/rule-width-interpolation-conversion-002.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/animation/rule-width-interpolation-conversion-002.html"
}
```

## style[0]

```css

    #target {
      column-rule-width: 10px, 10px;
      column-rule-style: solid;
      animation: width-anim 2s linear paused;
    }
    @keyframes width-anim {
      from { column-rule-width: 0px, repeat(auto, 0px); }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
