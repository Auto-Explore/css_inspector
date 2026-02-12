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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
