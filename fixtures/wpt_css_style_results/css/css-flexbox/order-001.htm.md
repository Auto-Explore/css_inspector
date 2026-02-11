# css/css-flexbox/order-001.htm

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/order-001.htm"
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
                height: 100px;
                width: 150px;
            }
            #flexItem1
            {
                background-color: orange;
            }
            #flexItem2
            {
                background-color: blue;
                order: -1;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
