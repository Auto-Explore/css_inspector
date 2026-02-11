# css/css-contain/contain-size-block-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-block-001-ref.html"
}
```

## style[0]

```css

  .basic {
    border: 1em solid green;
  }
  .height-ref {
    height: 50px;
    background: lightblue;
  }
  .width-ref {
    width: 50px;
  }
  .floatLBasic-ref {
    float: left;
  }
  .floatLWidth-ref {
    float: left;
    width: 50px;
  }
  .flexBaselineCheck {
    display: flex;
    align-items: baseline;
  }
  .innerContents {
    color: transparent;
    width: 0;
    height: 0;
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
