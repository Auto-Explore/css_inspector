# css/css-contain/contain-size-grid-005.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-grid-005.html"
}
```

## style[0]

```css

  .contain {
    display: grid;
    contain:size;
    border: 1em solid green;
    background: red;
  }
  .innerContents {
    color: transparent;
    height: 100px;
    width: 100px;
  }
  .minHeight {
    min-height: 40px;
    background: lightblue;
  }
  .height {
    height: 40px;
    background: lightblue;
  }
  .maxWidth {
    max-width: 40px;
  }
  .width {
    width: 40px;
  }
  .floatLBasic {
    float: left;
  }
  .floatLWidth {
    float: left;
    width: 40px;
  }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
