# css/css-contain/contain-size-block-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-block-004-ref.html"
}
```

## style[0]

```css

  .basic {
    overflow: hidden;
    position: relative;
    border: 2px solid green;
  }
  .height-ref {
    height: 60px;
    background: lightblue;
  }
  .width-ref {
    width: 60px;
  }
  .floatLBasic-ref {
    float: left;
  }
  .floatLWidth-ref {
    float: left;
    width: 60px;
  }
  .flexBaselineCheck {
    display: flex;
    align-items: baseline;
  }
  .innerContents {
    color: transparent;
    height: 100px;
    width: 100px;
    position: absolute;
  }
  .zeroHeightContents {
    color: transparent;
    height: 0px;
    width: 0px;
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
