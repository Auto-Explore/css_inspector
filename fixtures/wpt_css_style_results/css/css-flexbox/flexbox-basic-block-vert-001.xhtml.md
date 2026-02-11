# css/css-flexbox/flexbox-basic-block-vert-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-basic-block-vert-001.xhtml"
}
```

## style[0]

```css

      div { width: 50px; }
      div.flexbox {
        float: left;
        border: 1px dashed blue;
        height: 200px;
        font-size: 10px;
        display: flex;
        flex-direction: column;
      }
      div.a {
        flex: 1 0 30px;
        background: lightgreen;
      }
      div.b {
        flex: 2 0 20px;
        background: yellow;
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
