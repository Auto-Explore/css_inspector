# css/css-flexbox/flexbox-align-self-horiz-001-block.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-001-block.xhtml"
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

        /* Any children whose align-self is 'auto' (or unspecified, or
           initial) will end up taking this value from us: */
        align-items: center;

        /* Any children whose align-self is 'inherit' will end up
           inheriting this value from us: */
        align-self: flex-end;
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
    
```

```json
{
  "errors": 5,
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
    }
  ],
  "warnings": 0
}
```
