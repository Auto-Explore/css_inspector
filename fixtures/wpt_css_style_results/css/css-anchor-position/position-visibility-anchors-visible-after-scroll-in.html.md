# css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in.html"
}
```

## style[0]

```css

  #scroll-container {
    overflow: hidden scroll;
    scrollbar-width: none;
    width: 300px;
    height: 100px;
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
    position-area: block-end center;
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
