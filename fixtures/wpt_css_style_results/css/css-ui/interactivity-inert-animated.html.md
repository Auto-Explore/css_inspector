# css/css-ui/interactivity-inert-animated.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/interactivity-inert-animated.html"
}
```

## style[0]

```css

  @keyframes --anim {
    0% { interactivity: auto; }
  }
  #inert {
    animation: --anim 10000s step-end;
    interactivity: inert;
    width: 100px;
    height: 100px;
    background: lime;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
