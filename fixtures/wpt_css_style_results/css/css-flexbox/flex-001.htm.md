# css/css-flexbox/flex-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-001.htm"
}
```

## style[0]

```css

            #flexbox
            {
                background-color: red;
                display: flex;
                width: 300px;
            }
            div div
            {
                background-color: orange;
                flex: 1 0 auto;
                height: 100px;
            }
            #flexItem1
            {
                background-color: blue;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
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
