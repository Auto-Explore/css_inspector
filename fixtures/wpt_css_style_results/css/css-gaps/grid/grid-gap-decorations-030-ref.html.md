# css/css-gaps/grid/grid-gap-decorations-030-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-030-ref.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .grid-container {
    display: grid;
    grid-gap: 10px;
    grid-template-columns: 100px 100px 100px;
    width: 120px;
    height: 120px;
  }

  .item {
    background: gray;
    opacity: 0.5;
  }

  .col-gap {
    position: absolute;
    top: 0px;
    width: 0px;
    height: 120px;
    border-left: solid 5px blue;
  }

  .col-gap1 {
    left: 102.5px;
  }

  .col-gap2 {
    left: 212.5px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
