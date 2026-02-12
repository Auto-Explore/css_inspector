# css/css-anchor-position/transform-013.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/transform-013.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 200px;
    height: 200px;
    translate: 0;
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
    from { translate: 0; }
    to { translate: 100px; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
