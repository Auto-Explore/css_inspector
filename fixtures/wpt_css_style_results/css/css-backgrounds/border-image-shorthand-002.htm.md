# css/css-backgrounds/border-image-shorthand-002.htm

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-shorthand-002.htm"
}
```

## style[0]

```css

            div
            {
                border-color: red;
                border-style: double;
                border-width: 20px;
                height: 100px;
                margin: 50px;
                width: 100px;
            }

            #test
            {
                /* We first set border-image-outset to an entirely arbitrary value */
                border-image-outset: 2;
                /*
                and then we check with a shorthand value (which is deliberately
                omitting the border-image-outset sub-property) if the border-image-outset
                sub-property gets reset to its initial default value
                (which is 0 and not 2)
                */
                border-image: url("support/9grid40-30-20-10-red-old.png") 40% 15% 20% 5% / 1 stretch;
            }

            #reference
            {
                border-image-outset: 0 0 0 0;
                border-image-repeat: stretch;
                border-image-slice: 40% 15% 20% 5%;
                border-image-source: url("support/9grid40-30-20-10-red-old.png");
                border-image-width: 1;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
