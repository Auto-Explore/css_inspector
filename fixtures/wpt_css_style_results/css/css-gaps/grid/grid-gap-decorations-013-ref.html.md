# css/css-gaps/grid/grid-gap-decorations-013-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-013-ref.html"
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
    height: 320px;
  }

  .item {
    background: gray;
    opacity: 0.5;
  }

  .row-gap {
    width: 320px;
    height: 0px;
    border-bottom: solid 5px red;
  }

  .row-gap1 {
    position: absolute;
    top: 102.5px;
  }

  .row-gap2 {
    position: absolute;
    top: 212.5px;
  }

  .col-gap1 {
    position: absolute;
    width: 0;
    display: flex;
    gap: 20px;
    flex-direction: column;
    top: 5px;
    left: 102.5px;
  }

  .col-gap1>div {
    height: 90px;
    border-left: solid 5px blue;
  }

  .col-gap2 {
    position: absolute;
    width: 0;
    display: flex;
    gap: 20px;
    flex-direction: column;
    top: 5px;
    left: 212.5px;
  }

  .col-gap2>div {
    height: 90px;
    border-left: solid 5px blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
