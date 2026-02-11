# css/css-animations/responsive/column-rule-color-001.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/responsive/column-rule-color-001.html"
}
```

## style[0]

```css

  .paused {
    animation-duration: 4s;
    animation-timing-function: linear;
    animation-delay: -2s;
    animation-play-state: paused;
  }
  #container {
    color: rgb(80, 0, 0);
  }
  #first {
    animation-name: first-anim;
    color: rgb(60, 0, 0);
  }
  #second {
    animation-name: second-anim;
  }
  @keyframes first-anim {
    from { column-rule-color: currentColor; }
    to { column-rule-color: rgb(0, 60, 0); }
  }
  @keyframes second-anim {
    from { column-rule-color: inherit; }
    to { column-rule-color: rgb(0, 0, 80); }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
