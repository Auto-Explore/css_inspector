# css/css-flexbox/flexbox-align-self-horiz-003.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-003.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        padding: 3px;
        height: 4px;
        display: inline-flex;
        font-size: 10px;
        line-height: 10px;
        font-family: sans-serif;
      }

      .flexbox > div {
        width: 40px;
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
      .auto {
        background: yellow;
        align-self: auto;
      }
      .unspecified {
        background: lightgreen;
      }
      .initial {
        background: aqua;
        align-self: initial;
      }
      .inherit {
        background: violet;
        align-self: inherit;
      }
      .normal {
        background: tan;
        align-self: normal;
      }
   
```

```json
{
  "errors": 6,
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
    },
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
