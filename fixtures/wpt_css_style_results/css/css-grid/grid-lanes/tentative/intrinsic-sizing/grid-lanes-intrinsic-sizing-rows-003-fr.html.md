# css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-003-fr.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/intrinsic-sizing/grid-lanes-intrinsic-sizing-rows-003-fr.html"
}
```

## style[0]

```css


@import "support/grid-lanes-intrinsic-sizing-visual.css";

grid {
  display: inline-grid-lanes;
  grid-lanes-direction: row;
  gap: 1px 2px;
  grid-template-rows: 1fr 2fr 1fr 1fr;
  border: 1px solid;
  padding: 1px 0 2px 0;
  vertical-align: top;
  height: min-content;
  font-family: Ahem;
}

item {
  /* allow for differing min-content and max-content sizes */
  writing-mode: vertical-rl;
  text-orientation: sideways;
}

grid > item:nth-child(1) {
  background-color: #89CFF0;
}

grid > item:nth-child(2) {
  background-color: #FF6F61;
}

grid > item:nth-child(3) {
  background-color: #FDF3E7;
}

grid > item:nth-child(4) {
  background-color: #F4C542;
}

grid > item:nth-child(5) {
  background-color: #333333;
}

grid > item:nth-child(6) {
  background-color: #B2C8A5;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
