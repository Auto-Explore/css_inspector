# css/css-position/multicol/vrl-rtl-rtl-in-multicols.html

```json
{
  "format_version": 3,
  "file": "css/css-position/multicol/vrl-rtl-rtl-in-multicols.html"
}
```

## style[0]

```css

  body { writing-mode: vertical-rl; }
  .container {
    position: relative;
    background: green;
    font: 20px/1 Ahem;
    inline-size: 80px;
    block-size: 120px;
    color: green;
  }
  .multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0;
    block-size: 60px;
    inline-size: 160px;
  }
  .red { color: red; }
  .cb { position: relative; }
  .rtl { direction: rtl; }
  .ltr { direction: ltr; }
  .inline { display: inline; }
  .abs { position: absolute; }
  .inset-start { inset-block-start: 0; inset-inline-start: 0; }
  .inset-end { inset-block-end: 0; inset-inline-end: 0; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
