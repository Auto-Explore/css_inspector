# css/css-flexbox/justify-content-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/justify-content-001.htm"
}
```

## style[0]

```css

            #flexbox
            {
                background: linear-gradient(to right, blue 0, blue 75px, red 75px, red 225px, orange 225px, orange 300px);
                display: flex;
                justify-content: center;
                height: 100px;
                width: 300px;
            }
            div div
            {
                background-color: orange;
                width: 76px;
            }
            .blue
            {
                background-color: blue;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
