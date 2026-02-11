# css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-ref.html"
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

  #target {
    width: 100px;
    height: 100px;
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
