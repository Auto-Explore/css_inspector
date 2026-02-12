# css/css-conditional/container-queries/percentage-padding-orthogonal.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/percentage-padding-orthogonal.html"
}
```

## style[0]

```css

  #vertical {
    width: 500px;
    writing-mode: vertical-lr;
  }

  #padded {
    width: 100%;
    box-sizing: border-box;
    padding-right: 100%;
    background-color: orange;
  }

  #horizontal {
    writing-mode: horizontal-tb;
    width: 100%;
  }

  #container {
    width: 100%;
    container-type: inline-size;
    background-color: green;
  }

  #first, #second { height: 50px; }

  @container (width <= 400px) {
    #second { display: none; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
