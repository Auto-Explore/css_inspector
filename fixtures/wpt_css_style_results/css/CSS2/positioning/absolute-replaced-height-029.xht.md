# css/CSS2/positioning/absolute-replaced-height-029.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-029.xht"
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
                bottom: 0.5in;
                margin-bottom: 0.5in;
                margin-top: 0.5in;
                position: absolute;
                top: 0.5in;
            }
            div div
            {
                background: blue;
                height: 15px;
                left: 15px;
                position: relative;
                top: 1in;
                width: 15px;
            }

	/*

  			0.5in : 'top'
		+
  			0.5in : 'margin-top'
		+
  			0     : 'border-top-width'
		+
  			0     : 'padding-top'
		+
  			15px  : 'height' (intrinsic height of inline replaced element)
		+
  			0     : 'padding-bottom'
		+
  			0     : 'border-bottom-width'
		+
  			0.5in : 'margin-bottom'
		+
  			0.5in : 'bottom'
		===========
  			207px  : height of containing block (15px)

		So, bottom has to be ignored and forced to have the value that
		will balance the equation. So, this brings up

			0.5in : 'top'
		+
  			0.5in : 'margin-top'
		+
			0     : 'border-top-width'
		+
			0     : 'padding-top'
		+
			15px  : 'height' (intrinsic height)
		+
			0     : 'padding-bottom'
		+
			0     : 'border-bottom-width'
		+
			0.5in : 'margin-bottom'
		+
 			(solve): 'bottom'
		===================
  			15px  : height of containing block

		So, the solved bottom value must be -1.5in (or -144px).
	*/
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
