# css/css-flexbox/flex-container-max-content-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-container-max-content-001-ref.html"
}
```

## style[0]

```css

@import "/fonts/ahem.css"; /* optional */

body {
  /* Fit it in 800x600 pixels */
  display: grid;
  grid-template-columns: repeat(auto-fill, 66px 66px 66px);
  grid-auto-rows: 50px;
  font: 10px/1 Ahem, monospace;
}

.wrap {
  counter-increment: test;
}

.row, .col {
  background: blue;
  padding: 5px;
  float: left;
}

.item {
  padding: 3px;
  border: 2px solid aqua;
  color: orange;
}


/* help people debugging */
.wrap:hover::before {
  content: counter(test, decimal-leading-zero);
  position: absolute;
  font: initial;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
