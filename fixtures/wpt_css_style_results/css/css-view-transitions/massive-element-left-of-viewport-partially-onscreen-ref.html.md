# css/css-view-transitions/massive-element-left-of-viewport-partially-onscreen-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/massive-element-left-of-viewport-partially-onscreen-ref.html"
}
```

## style[0]

```css

:root {
  writing-mode: vertical-lr;
  font: 12px/1 Ahem;
}

.target {
  contain: paint;
  inline-size: 100px;
  block-size: 40000px;
  view-transition-name: target;
}

.top {
  inline-size: 100%;
  block-size: 100px;
  background: lightblue;
}

.middle {
  inline-size: 100%;
  block-size: 39800px;
  background: green;
}

.bottom {
  inline-size: 100%;
  block-size: 100px;
  background: blue;
}

.hidden {
  contain: paint;
  inline-size: 10px;
  block-size: 10px;
  background: grey;
  visibility: hidden;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
