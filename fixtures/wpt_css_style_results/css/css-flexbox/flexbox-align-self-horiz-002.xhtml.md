# css/css-flexbox/flexbox-align-self-horiz-002.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-002.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        height: 200px;
        display: inline-flex;
        font-size: 10px;
        line-height: 10px;
        vertical-align: top;
      }

      .flexbox > div {
        width: 40px;
        margin:       1px 2px 3px 4px;
        border-width: 2px 3px 4px 5px;
        padding:      3px 4px 5px 6px;
        border-style: dotted;
      }

      .big {
        height: 100px;
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
      .self-start {
        background: yellow;
        align-self: self-start;
      }
      .self-end {
        background: purple;
        align-self: self-end;
      }
      .wmvertrev {
        writing-mode: vertical-lr;
        direction: rtl;
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
