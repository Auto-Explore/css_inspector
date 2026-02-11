# css/css-gaps/grid/grid-gap-decorations-031-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-031-ref.html"
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
    grid-template-rows: 100px 100px 100px;
    grid-auto-flow: column;
    width: 120px;
    height: 120px;
  }

  .item {
    background: gray;
    opacity: 0.5;
  }

  .row-gap {
    position: absolute;
    width: 120px;
    height: 0px;
    border-bottom: solid 5px red;
  }

  .row-gap1 {
    top: 102.5px;
  }

  .row-gap2 {
    top: 212.5px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
