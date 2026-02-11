# css/css-flexbox/flexbox-mbp-horiz-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-001.xhtml"
}
```

## style[0]

```css

      div { height: 100px; border: 0; }
      div.flexbox {
        width: 200px;
        font-size: 10px;
        display: flex;
      }
      div.a {
        flex: 1 0 24px;
        background: lightgreen;
        border-style: dotted;
        border-left-width: 2px;
        border-right-width: 4px;
      }
      div.b {
        flex: 2 0 10px;
        background: yellow;
        border-style: dashed;
        border-left-width: 7px;
        border-right-width: 3px;
      }
      div.c {
        flex: 3 0 40px;
        background: orange;
      }
      div.flexNone {
        flex: none;
        background: pink;
      }
      div.flexBasis {
        flex: 0 0 20px;
        background: gray;
      }
      div.spacer {
        display: inline-block;
        width: 15px;
        height: 15px;
        background: purple;
      }
    
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
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
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
