# css/css-align/self-alignment/self-align-safe-unsafe-flex-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-align/self-alignment/self-align-safe-unsafe-flex-001-ref.html"
}
```

## style[0]

```css

  /* Label things */
  body > div {
    display: flow-root;
  }
  body > div::before {
    display: block;
    content: attr(class);
    font-size: 10px;
    margin-top: 10px;
  }

  /* Test Framework */
  .container {
    border: solid aqua;
    margin: 10px;
    float: left;
    width: 30px;
    height: 30px;
  }

  .item {
    background: orange;
    width: 100%;
  }

  /* Test */
  .small .item {
    height: 20px;
  }
  .large .item {
    height: 40px;
  }

  .small .center
    { margin-top: 5px }
  .small .end,
  .small .self-end,
  .small .flex-end
    { margin-top: 10px }
  .large .center
    { margin-top: -5px; }
  .large .end,
  .large .self-end,
  .large .flex-end
    { margin-top: -10px; }
  .safe .item
    { margin-top: 0; }
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
