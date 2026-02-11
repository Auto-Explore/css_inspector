# css/CSS2/floats-clear/clear-float-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/clear-float-006.xht"
}
```

## style[0]

```css

            div
            {
                width: 3in;
            }
            span
            {
                height: 1in;
                width: 1in;
            }
            #span1, #span2
            {
                background-color: orange;
            }
            #span1
            {
                float: right;
            }
            #span2, #span3
            {
                float: left;
            }
            #span3
            {
                background: blue;
                clear: both;
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
