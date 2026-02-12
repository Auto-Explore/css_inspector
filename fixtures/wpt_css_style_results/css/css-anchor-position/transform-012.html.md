# css/css-anchor-position/transform-012.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-012.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 200px;
    height: 200px;
    scale: 0.5;
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
    from { scale: 0.5; }
    to { scale: 2; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
