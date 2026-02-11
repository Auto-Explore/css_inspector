# css/css-flexbox/align-content-005.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/align-content-005.htm"
}
```

## style[0]

```css

            #flexbox
            {
                background: linear-gradient(to bottom, green 0, green 15px, red 15px, red 35px, green 35px, green 65px, red 65px, red 85px, green 85px, green 100px);
                align-content: space-around;
                display: flex;
                flex-flow: wrap;
                height: 100px;
                width: 300px;
            }
            div div
            {
                background-color: green;
                height: 22px;
                width: 150px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
