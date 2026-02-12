# css/css-anchor-position/position-visibility-anchors-visible-change-css-visibility.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-change-css-visibility.html"
}
```

## style[0]

```css

  #container {
    visibility: hidden;
  }

  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
    background: orange;
  }

  #target {
    position-anchor: --a1;
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
