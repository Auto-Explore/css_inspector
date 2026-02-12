# css/css-anchor-position/position-visibility-anchors-visible-with-position.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-with-position.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    width: 300px;
    height: 100px;
    /* Same as position-visibility-anchors-visible.html, but with relpos here */
    position: relative;
  }

  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
  }

  #spacer {
    height: 100px;
  }

  #target {
    position-anchor: --a1;
    position-visibility: anchors-visible;
    position-area: bottom right;
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
