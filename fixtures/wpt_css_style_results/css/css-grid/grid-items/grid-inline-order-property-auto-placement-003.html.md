# css/css-grid/grid-items/grid-inline-order-property-auto-placement-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-order-property-auto-placement-003.html"
}
```

## style[0]

```css

  #reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
  }

  #inline-grid {
    display: inline-grid;
    font: 50px/1 Ahem;
    grid-template-columns: auto auto;
  }

  #blue {
    color: blue;
    order: -5;
  }

  #yellow {
    color: yellow;
    order: -5;
  }

  #lime {
    color: lime;
  }

  #magenta {
    color: magenta;
    order: 1;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
