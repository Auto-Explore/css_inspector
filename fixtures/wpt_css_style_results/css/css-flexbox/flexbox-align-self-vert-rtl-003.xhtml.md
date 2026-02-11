# css/css-flexbox/flexbox-align-self-vert-rtl-003.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-vert-rtl-003.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        width: 4px;
        display: flex;
        flex-direction: column;
        direction: rtl;
        font-family: sans-serif;
        font-size: 10px;
        margin-left: 80px;
      }

      div.big {
        font-size: 18px;
        width: 50px;
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
