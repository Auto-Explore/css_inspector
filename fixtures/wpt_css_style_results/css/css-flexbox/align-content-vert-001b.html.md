# css/css-flexbox/align-content-vert-001b.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-content-vert-001b.html"
}
```

## style[0]

```css

      div.flexbox {
        width: 200px;
        max-height: 10px; /* Short, to force us to wrap */
        display: flex;
        flex-direction: column;
        flex-wrap: wrap;
        margin-bottom: 2px;
        background: lightgray;
      }
      div.a {
        width: 10px;
        height: 10px;
        flex: none;
        background: lightgreen;
      }
      div.b {
        width: auto; /* width comes from contents */
        height: 10px;
        flex: none;
        background: pink;
      }
      div.c {
        width: 40px;
        height: 10px;
        flex: none;
        background: orange;
      }

      /* Inside of 'b': */
      div.fixedSizeChild {
        width: 30px;
        height: 5px;
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
