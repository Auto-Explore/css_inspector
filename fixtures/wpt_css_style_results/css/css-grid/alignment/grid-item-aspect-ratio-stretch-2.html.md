# css/css-grid/alignment/grid-item-aspect-ratio-stretch-2.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-item-aspect-ratio-stretch-2.html"
}
```

## style[0]

```css

  body {
    line-height: 0;
  }

  div {
    display: inline-grid;
    grid-template: 100% / 100%;
    height: 250px;
    width: 350px;
    background: grey;
    margin: 10px;
    vertical-align: top;
  }

  .justify {
    justify-self: stretch;
  }
  .align {
    align-self: stretch;
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
