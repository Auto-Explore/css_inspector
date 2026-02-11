# css/css-align/self-alignment/self-align-safe-unsafe-grid-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-align/self-alignment/self-align-safe-unsafe-grid-002-ref.html"
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
    height: 100%;
  }

  /* Test */
  .small .item {
    width: 20px;
  }
  .large .item {
    width: 40px;
  }

  .small .center
    { margin-left: 5px }
  .small .end,
  .small .self-end,
  .small .flex-end
    { margin-left: 10px }
  .large .center
    { margin-left: -5px; }
  .large .end,
  .large .self-end,
  .large .flex-end
    { margin-left: -10px; }
  .large.safe .item
    { margin-left: 0; }
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
