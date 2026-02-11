# css/css-contain/contain-layout-dynamic-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-dynamic-005.html"
}
```

## style[0]

```css

  div {
      font: 25px/1 Ahem;
  }
  #wrapper {
      width: 15em;
      background: lightgray;
  }
  .containment {
      contain: layout;
  }
  .float {
      box-sizing: border-box;
      border: 1px solid black;
  }
  .left {
      float: left;
      width: 1em;
      height: 2em;
  }
  .right {
      float: right;
      width: 1em;
      height: 3em;
  }
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
