# css/css-anchor-position/anchor-position-sibling-index.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-sibling-index.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #abs {
    position-anchor: --a;
    position: absolute;
    top: anchor(calc(25% * sibling-index()));
    width: 100px;
    height: 100px;
    background: teal;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
