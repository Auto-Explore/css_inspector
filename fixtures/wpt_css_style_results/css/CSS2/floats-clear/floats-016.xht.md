# css/CSS2/floats-clear/floats-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/floats-016.xht"
}
```

## style[0]

```css

            #container
            {
                margin: 0.5in;
            }
            #inline
            {
                background: yellow;
                margin-left: -0.5in;
            }
            #float, #block
            {
                height: 1in;
                width: 1in;
            }
            #float
            {
                background: orange;
                float: left;
            }
            #block
            {
                background: blue;
                margin: -0.25in;
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
