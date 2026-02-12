# css/css-anchor-position/anchor-center-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-scroll.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #scroller {
    width: 400px;
    height: 400px;
    overflow: auto;
    background: orange;
  }
  #anchor {
    margin-top: 100px;
    width: 100px;
    height: 100px;
    background: pink;
    anchor-name: --anchor;
  }
  #anchored {
    position: absolute;
    position-anchor: --anchor;
    align-self: anchor-center;
    width: 50px;
    height: 50px;
    background: purple;
  }
  #filler { height: 1000px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
