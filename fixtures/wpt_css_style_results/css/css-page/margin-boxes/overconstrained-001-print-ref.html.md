# css/css-page/margin-boxes/overconstrained-001-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/overconstrained-001-print-ref.html"
}
```

## style[0]

```css

  @page {
    margin: 0;
    size: 500px 400px;
  }
  body {
    display: grid;
    grid-template-columns: 100px auto 100px;
    grid-template-rows: 100px auto 100px;
    height: 100vh;
    margin: 0;
  }
  .square {
    border: solid;
    width: 25px;
    height: 25px;
    margin: 3px;
  }
  .square.left {
    margin-left: auto;
  }
  .square.top {
    margin-top: auto;
  }
  .vertical-edge {
    display: flex;
    justify-content: space-between;
  }
  .horizontal-edge {
    display: flex;
    flex-flow: column;
    justify-content: space-between;
  }
  .pagearea {
    border: solid blue;
    padding: 8px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
