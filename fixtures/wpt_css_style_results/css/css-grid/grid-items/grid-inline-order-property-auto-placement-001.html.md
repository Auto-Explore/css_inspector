# css/css-grid/grid-items/grid-inline-order-property-auto-placement-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-order-property-auto-placement-001.html"
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
  }

  #yellow {
    color: yellow;
    order: 1;
  }

  #lime {
    color: lime;
    order: 5;
  }

  #magenta {
    color: magenta;
    order: 10;
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
