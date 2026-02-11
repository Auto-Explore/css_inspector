# css/css-flexbox/flex-004.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-004.htm"
}
```

## style[0]

```css

            div
            {
                height: 50px;
            }
            #flexbox
            {
                background-color: red;
                display: flex;
                width: 300px;
            }
            #flexItem1
            {
                flex: 0 2 auto;
                width: 300px;
            }
            #flexItem2
            {
                flex: 0 3 auto;
                width: 600px;
            }
            #flexItem1, #ref1
            {
                background-color: blue;
            }
            #flexItem2, #ref2
            {
                background-color: orange;
            }
            #ref1, #ref2
            {
                display: inline-block;
                width: 150px;
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
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
