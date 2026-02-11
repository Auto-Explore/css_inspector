# css/css-flexbox/flexbox-justify-content-vert-006.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-justify-content-vert-006.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        height: 200px;
        display: flex;
        flex-direction: column-reverse;
        margin-right: 2px;
        float: left;
      }
      div.a {
        width: 20px;
        flex: 0 10px;
        background: lightgreen;
      }
      div.b {
        width: 20px;
        flex: 0 50px;
        background: pink;
      }
      div.c {
        width: 20px;
        flex: 0 100px;
        background: orange;
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
