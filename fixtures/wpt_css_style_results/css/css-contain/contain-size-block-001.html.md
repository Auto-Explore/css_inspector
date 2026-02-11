# css/css-contain/contain-size-block-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-block-001.html"
}
```

## style[0]

```css

  .contain {
    contain: size;
    border: 1em solid green;
    background: red;
  }
  .innerContents {
    color: transparent;
    height: 100px;
    width: 100px;
  }
  .minHeight {
    min-height: 50px;
    background: lightblue;
  }
  .height {
    height: 50px;
    background: lightblue;
  }
  .maxWidth {
    max-width: 50px;
  }
  .width {
    width: 50px;
  }
  .floatLBasic {
    float: left;
  }
  .floatLWidth {
    float: left;
    width: 50px;
  }
  .flexBaselineCheck {
    display: flex;
    align-items: baseline;
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
