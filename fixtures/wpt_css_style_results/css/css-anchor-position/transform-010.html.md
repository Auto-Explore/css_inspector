# css/css-anchor-position/transform-010.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-010.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 200px;
    height: 200px;
    transform: translateX(0);
    background: hotpink;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: bottom right;
    width: 100%;
    height: 100%;
    background: cyan;
  }
  @keyframes anim {
    from { transform: translateX(0) }
    to { transform: translateX(200px) }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
