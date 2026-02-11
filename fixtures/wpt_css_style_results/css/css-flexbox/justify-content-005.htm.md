# css/css-flexbox/justify-content-005.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/justify-content-005.htm"
}
```

## style[0]

```css

            #flexbox
            {
                background: linear-gradient(to right, blue 0, blue 38px, red 38px, red 112px, blue 112px, blue 150px, orange 150px, orange 188px, red 188px, red 262px, orange 262px, orange 300px);
                display: flex;
                justify-content: space-around;
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
