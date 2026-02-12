# css/css-transforms/animation/singular-scale-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/animation/singular-scale-nested.html"
}
```

## style[0]

```css

  .menu {
    width: 100px;
    scale: 0;
  }
  @keyframes anim {
    0% { scale: 0; }
    0.1% { scale: 1; }
    100% { scale: 1; }
  }
  .trigger > .menu {
    animation: anim 100s linear 1 forwards;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
