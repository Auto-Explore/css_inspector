# css/css-align/self-alignment/self-align-safe-unsafe-flex-003.html

```json
{
  "format_version": 3,
  "file": "css/css-align/self-alignment/self-align-safe-unsafe-flex-003.html"
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
    float: right;
    width: 30px;
    height: 30px;
    display: flex;
    flex-flow: column;
  }

  .item {
    background: orange;
    flex: none;
    height: 100%;
  }

  /* Test */
  .small .item {
    width: 20px;
  }
  .large .item {
    width: 40px;
  }

  .center     { align-self: center; }
  .start      { align-self: start; }
  .end        { align-self: end; }
  .self-start { align-self: self-start; }
  .self-end   { align-self: self-end; }
  .flex-start { align-self: flex-start; }
  .flex-end   { align-self: flex-end; }

  .safe .center     { align-self: safe center; }
  .safe .start      { align-self: safe start; }
  .safe .end        { align-self: safe end; }
  .safe .self-start { align-self: safe self-start; }
  .safe .self-end   { align-self: safe self-end; }
  .safe .flex-start { align-self: safe flex-start; }
  .safe .flex-end   { align-self: safe flex-end; }

  .safe .center     { align-self: safe center; }
  .safe .start      { align-self: safe start; }
  .safe .end        { align-self: safe end; }
  .safe .self-start { align-self: safe self-start; }
  .safe .self-end   { align-self: safe self-end; }
  .safe .flex-start { align-self: safe flex-start; }
  .safe .flex-end   { align-self: safe flex-end; }
```

```json
{
  "errors": 15,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “align-self”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
