# css/css-flexbox/flexbox-justify-content-horiz-002.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-justify-content-horiz-002.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        width: 200px;
        display: flex;
        margin-bottom: 4px;
        border: 1px dotted black;
      }
      div.a {
        height: 10px;
        flex: 0 10px;
        background: lightgreen;
        border-style: solid;
        border-color:     purple;
        border-top-width:    1px;
        border-right-width:  2px;
        border-bottom-width: 3px;
        border-left-width:   4px;
        padding: 2px;
      }
      div.b {
        height: 10px;
        flex: 0 50px;
        background: pink;
        padding: 4px 3px 2px 1px;
        margin: 2px 3px 4px 5px;
      }
      div.c {
        height: 10px;
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
