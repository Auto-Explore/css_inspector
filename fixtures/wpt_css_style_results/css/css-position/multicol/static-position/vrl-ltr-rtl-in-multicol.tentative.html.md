# css/css-position/multicol/static-position/vrl-ltr-rtl-in-multicol.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-position/multicol/static-position/vrl-ltr-rtl-in-multicol.tentative.html"
}
```

## style[0]

```css

  body { writing-mode: vertical-rl; }
  .container {
    position: relative;
    background: green;
    font: 16px/1 Ahem;
    inline-size: 80px;
    block-size: 100px;
    color: green;
  }
  .multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0;
    block-size: 100px;
    inline-size: 160px;
  }
  .red { color: red; }
  .cb {
    position: relative;
    inset-block-start: 15px;
    inset-inline-start: 20px;
  }
  .rtl { direction: rtl; }
  .ltr { direction: ltr; }
  .inline { display: inline; }
  .abs { position: absolute; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
