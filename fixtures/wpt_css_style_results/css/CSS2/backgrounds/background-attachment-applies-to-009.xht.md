# css/CSS2/backgrounds/background-attachment-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-attachment-applies-to-009.xht"
}
```

## style[0]

```css

            p {font: 1em/1.25 serif;}

            span
            {
                background-color: orange;
                background-image: url('support/blue96x96.png');
                background-attachment: fixed;
                background-repeat: repeat-x;
                display: block;
                height: 2in;
                width: 1in;
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
