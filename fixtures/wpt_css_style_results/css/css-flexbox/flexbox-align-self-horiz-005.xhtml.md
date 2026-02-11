# css/css-flexbox/flexbox-align-self-horiz-005.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-005.xhtml"
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

      .kidsAutoTop > div    { margin-top: auto;    }
      .kidsAutoBottom > div { margin-bottom: auto; }
      .kidsAutoBoth > div   { margin: auto 0; }

      .flexbox > div {
        width: 40px;
      }

      .flexbox > div.big {
        height: 80px;
        font-size: 20px;
        line-height: 20px;
      }

      /* Classes for each of the various align-self values */
      .flex-start {
        background: lime;
        align-self: flex-start;
      }
      .flex-end {
        background: orange;
        align-self: flex-end;
      }
      .center {
        background: lightblue;
        align-self: center;
      }
      .baseline {
        background: teal;
        align-self: baseline;
      }
      .stretch {
        background: pink;
        align-self: stretch;
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
