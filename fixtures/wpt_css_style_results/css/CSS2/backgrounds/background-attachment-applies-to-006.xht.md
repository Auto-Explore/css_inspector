# css/CSS2/backgrounds/background-attachment-applies-to-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-attachment-applies-to-006.xht"
}
```

## style[0]

```css

            p {font: 1em/1.25 serif;}

            #test
            {
                background-color: orange;
                background-image: url('support/blue96x96.png');
                background-attachment: fixed;
                background-repeat: repeat-x;
                display: table-column;
            }
            #table
            {
                display: table;
                table-layout: fixed;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
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
