# css/css-grid/grid-items/grid-inline-order-property-auto-placement-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-inline-order-property-auto-placement-005.html"
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
    order: -1;
  }

  #lime {
    color: lime;
    order: 1;
  }

  #magenta {
    color: magenta;
    order: 5;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
