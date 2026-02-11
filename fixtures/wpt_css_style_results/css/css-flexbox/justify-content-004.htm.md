# css/css-flexbox/justify-content-004.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/justify-content-004.htm"
}
```

## style[0]

```css

            #flexbox
            {
                background: linear-gradient(to right, red 0, red 75px, blue 75px, blue 150px, orange 150px, orange 225px, red 225px, red 300px);
                display: flex;
                justify-content: space-between;
                height: 100px;
                width: 300px;
            }
            div div
            {
                background-color: orange;
                width: 76px;
            }
            #blue
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
