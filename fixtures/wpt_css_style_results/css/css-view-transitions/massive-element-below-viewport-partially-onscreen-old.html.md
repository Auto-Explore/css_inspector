# css/css-view-transitions/massive-element-below-viewport-partially-onscreen-old.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/massive-element-below-viewport-partially-onscreen-old.html"
}
```

## style[0]

```css

:root {
  font: 12px/1 Ahem;
}

.target {
  inline-size: 100px;
  block-size: 40000px;
  view-transition-name: target;
}

.top {
  /* Ahem font sometimes causes slight difference between actual and reference
    images due to anti-aliasing when bordering the white background in this
    case. Adding a padding reduces the test flakiness. */
  padding-top: 1px;
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
  view-transition-name: hidden;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

html::view-transition-old(*), html::view-transition-new(*) {
  object-fit: none;
}

html::view-transition-old(target) { animation: unset; opacity: 1; }
html::view-transition-new(target) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 9,
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
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
