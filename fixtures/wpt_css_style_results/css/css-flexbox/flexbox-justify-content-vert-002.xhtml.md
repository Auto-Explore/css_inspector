# css/css-flexbox/flexbox-justify-content-vert-002.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-justify-content-vert-002.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        height: 200px;
        display: flex;
        flex-direction: column;
        margin-right: 2px;
        float: left;
        border: 1px dotted black;
      }
      div.a {
        width: 10px;
        flex: 0 10px;
        background: lightgreen;
        border-style: solid;
        border-color:     purple;
        border-top-width:    4px;
        border-right-width:  3px;
        border-bottom-width: 2px;
        border-left-width:   1px;
        padding: 2px;
      }
      div.b {
        width: 10px;
        flex: 0 50px;
        background: pink;
        padding: 1px 2px 3px 4px;
        margin: 5px 4px 3px 2px;
      }
      div.c {
        width: 10px;
        flex: 0 100px;
        background: orange;
        margin: 3px;
        border: 2px dashed teal;
      }
    
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
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
