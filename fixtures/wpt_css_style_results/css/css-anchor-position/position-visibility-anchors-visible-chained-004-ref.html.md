# css/css-anchor-position/position-visibility-anchors-visible-chained-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-chained-004-ref.html"
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
    width: 100px;
    height: 100px;
    background: orange;
    margin-bottom: 100px;
  }

  #chained {
    width: 100px;
    height: 50px;
    background: blue;
  }

  #target {
    width: 100px;
    height: 50px;
    background: green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
