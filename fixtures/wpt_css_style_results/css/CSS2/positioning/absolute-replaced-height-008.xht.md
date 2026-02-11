# css/CSS2/positioning/absolute-replaced-height-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-008.xht"
}
```

## style[0]

```css

            #div1
            {
                border-top: solid black;
                position: relative;
            }
            img
            {
                bottom: auto;
                height: auto;
                margin-bottom: auto;
                margin-top: auto;
                position: absolute;
                top: 1in;
                width: auto;
            }

			/*
			"
			The used value of 'height' is determined as for inline replaced elements.
			If 'margin-top' or 'margin-bottom' is specified as 'auto'
			its used value is determined by the rules below.
			(...)
			If 'height' and 'width' both have computed values
			of 'auto' and the element also has an intrinsic height,
			then that intrinsic height is the used value of 'height'.
			(...)
			If 'bottom' is 'auto', replace any 'auto' on 'margin-top'
			or 'margin-bottom' with '0'.
			If at this point there is only one 'auto' left,
			solve the equation for that value.
			"
			http://www.w3.org/TR/CSS21/visudet.html#inline-replaced-height

			In this test, bottom will be -96px because the height of the
			containing block is 0px.
			*/

            div div
            {
                background: blue;
                height: 15px;
                left: 15px;
                position: relative;
                top: 1in;
                width: 15px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
