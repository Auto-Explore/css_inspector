# css/CSS2/positioning/absolute-non-replaced-height-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-012.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                height: 3in;
                position: relative;
                width: 3in;
            }
            div div
            {
                position: absolute;
                top: 1in;
                bottom: auto;
                height: 1in;
                margin-top: auto;
                margin-bottom: auto;
                background: blue;
                width: 100%;
            }

            /*
			"
            'bottom' is 'auto', 'top' and 'height' are not 'auto',
            then set 'auto' values for 'margin-top' and 'margin-bottom' to 0
            and solve for 'bottom'
			"

			Therefore, bottom used value must be 1in
            */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
