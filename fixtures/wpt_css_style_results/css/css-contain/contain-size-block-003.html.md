# css/css-contain/contain-size-block-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-block-003.html"
}
```

## style[0]

```css

  .contain {
    contain: size;
    overflow: auto;
    border: 2px solid green;
  }
  .innerContents {
    color: transparent;
    height: 100px;
    width: 100px;
  }
  .minHeight {
    min-height: 60px;
    background: lightblue;
  }
  .height {
    height: 60px;
    background: lightblue;
  }
  .maxWidth {
    max-width: 60px;
  }
  .width {
    width: 60px;
  }
  .floatLBasic {
    float: left;
  }
  .floatLWidth {
    float: left;
    width: 60px;
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
