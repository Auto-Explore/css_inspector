# css/css-page/margin-boxes/auto-margins-002-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/auto-margins-002-print-ref.html"
}
```

## style[0]

```css

  @page {
    margin: 0;
    size: 500px 440px;
  }
  :root {
    print-color-adjust: exact;
  }
  body {
    display: grid;
    grid-template-columns: 100px auto 100px;
    grid-template-rows: 100px auto 100px;
    height: 100vh;
    margin: 0;
  }
  .square {
    box-sizing: border-box;
    width: 100px;
    border: solid;
    flex: 1;
    background: green;
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
    border: solid;
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
