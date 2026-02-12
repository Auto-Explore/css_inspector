# css/css-anchor-position/position-visibility-anchors-visible-change-anchor.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-change-anchor.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
  }

  .anchor {
    width: 100px;
    height: 100px;
    background: orange;
    display: inline-block;
  }

  #anchor1 {
    height: 200px;
    anchor-name: --a1;
  }

  #anchor2 {
    anchor-name: --a2;
  }

  #target {
    position-anchor: --a2;
    position-visibility: anchors-visible;
    position-area: bottom center;
    width: 100px;
    height: 100px;
    background: green;
    position: absolute;
    inset: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
