# css/css-flexbox/flexbox-align-self-horiz-005-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-005-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        height: 140px;
        width: 400px;
        display: flex;
        font-size: 10px;
        line-height: 10px;
        margin-bottom: 10px;
      }

      .kidsAutoTop > div      { margin-top: 130px;   }
      .kidsAutoTop > div.big  { margin-top: 60px;   }
      .kidsAutoBoth > div     { margin-top: 65px; }
      .kidsAutoBoth > div.big { margin-top: 30px; }

      .flexbox > div {
        width: 40px;
        height: 10px;
      }

      .flexbox > div.big {
        height: 80px;
        font-size: 20px;
        line-height: 20px;
      }

      /* Classes for each of the various align-self values */
      .flex-start {
        background: lime;
      }
      .flex-end {
        background: orange;
      }
      .center {
        background: lightblue;
      }
      .baseline {
        background: teal;
      }
      .stretch {
        background: pink;
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
