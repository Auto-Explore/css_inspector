# css/CSS2/box-display/containing-block-029.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/containing-block-029.xht"
}
```

## style[0]

```css

            div
            {
                background: blue;
                height: 1in;
                position: absolute;
                width: 1in;
            }
            div div:before
            {
                content: ".";
                background: orange;
                bottom: 0;
                display: block;
                height: 0.25in;
                position: absolute;
                right: 0;
                width: 0.25in;
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
