# css/css-flexbox/align-content-horiz-001b.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-content-horiz-001b.html"
}
```

## style[0]

```css

      div.flexbox {
        max-width: 20px; /* Skinny, to force us to wrap */
        height: 200px;
        display: flex;
        flex-wrap: wrap;
        margin-right: 2px;
        float: left;
        background: lightgray;
      }
      div.a {
        height: 10px;
        width: 20px;
        flex: none;
        background: lightgreen;
      }
      div.b {
        height: auto; /* height comes from contents */
        width: 20px;
        flex: none;
        background: pink;
      }
      div.c {
        height: 40px;
        width: 20px;
        flex: none;
        background: orange;
      }

      /* Inside of 'b': */
      div.fixedSizeChild {
        width: 10px;
        height: 30px;
        background: purple;
      }
    
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
