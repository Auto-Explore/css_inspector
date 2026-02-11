# css/CSS2/floats-clear/clear-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/clear-003.xht"
}
```

## style[0]

```css

            #div1
            {
                width: 2in;
            }
            div div
            {
                height: 1in;
                width: 1in;
            }
            #div2
            {
                background: blue;
                float: left;
            }
            #div3
            {
                float: right;
                background: orange;
                height: 1.5in;
            }
            #div4
            {
                background: black;
                clear: both;
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
