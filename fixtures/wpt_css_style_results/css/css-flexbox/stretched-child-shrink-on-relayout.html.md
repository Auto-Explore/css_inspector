# css/css-flexbox/stretched-child-shrink-on-relayout.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/stretched-child-shrink-on-relayout.html"
}
```

## style[0]

```css

.flexbox {
    display: flex;
    background: papayawhip;
    border: 1px solid midnightblue;
    margin: 1em;
    width: 50px;
}

.flexbox > div {
    margin: 5px;
    min-width: 10px;
    min-height: 10px;
    background-color: lawngreen;
}

.column {
    flex-flow: column;
}

.horizontal-tb {
    writing-mode: horizontal-tb;
}
.vertical-lr {
    writing-mode: vertical-lr;
}

.largeitem {
    height: 200px;
    width: 200px;
    margin: 5px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
