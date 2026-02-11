# css/css-backgrounds/border-image-shorthand-003.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-shorthand-003.htm"
}
```

## style[0]

```css

            div
            {
                border-color: red;
                border-style: double;
                border-width: 20px;
                height: 5px;
                margin: 100px;
                width: 5px;
            }

            #test
            {
                /* We first set border-image-width to an entirely arbitrary value */
                border-image-width: 3;
                /*
                and then we check with a shorthand value (which is deliberately
                omitting the border-image-width sub-property) if the border-image-width
                sub-property gets reset to its initial default value
                (which is 1 and not 3)
                */
                border-image: url("support/9grid40-30-20-10-red-old.png") 40% 15% 20% 5% / / 2 stretch;
            }

            #reference
            {
                border-image-outset: 2;
                border-image-repeat: stretch;
                border-image-slice: 40% 15% 20% 5%;
                border-image-source: url("support/9grid40-30-20-10-red-old.png");
                border-image-width: 1;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
