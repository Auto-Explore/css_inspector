# css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-document-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-visibility-anchors-visible-after-scroll-in-document-ref.html"
}
```

## style[0]

```css

  #anchor {
    width: 100px;
    height: 100px;
    margin-left: 150vw;
    background: orange;
  }

  #target {
    width: 100px;
    height: 100px;
    margin-top: -100px;
    margin-left: calc(150vw - 100px);
    background: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
