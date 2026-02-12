# css/css-grid/grid-items/grid-order-property-auto-placement-004.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-order-property-auto-placement-004.html"
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

  #grid {
    display: grid;
    font: 50px/1 Ahem;
    grid-template-columns: auto auto;
    justify-content: start;
    align-content: start;
  }

  #blue {
    color: blue;
    order: 1;
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
