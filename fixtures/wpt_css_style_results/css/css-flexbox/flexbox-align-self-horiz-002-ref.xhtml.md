# css/css-flexbox/flexbox-align-self-horiz-002-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-horiz-002-ref.xhtml"
}
```

## style[0]

```css

      .flexbox {
        border: 1px dashed blue;
        height: 200px;
        display: inline-block;
        font-size: 10px;
        line-height: 10px;
        vertical-align: top;
      }

      .flexbox > div {  float: left }
      .flex-start, .flex-end, .center, .baseline, .stretch,
      .auto, .unspecified, .initial, .inherit, .self-start, .self-end {
        width: 40px;
        margin:       1px 2px 3px 4px;
        border-width: 2px 3px 4px 5px;
        padding:      3px 4px 5px 6px;
        position: relative;
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
      .self-start {
        background: yellow;
      }
      .self-end {
        background: purple;
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
