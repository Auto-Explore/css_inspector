# css/CSS2/positioning/absolute-replaced-height-031.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-031.xht"
}
```

## style[0]

```css

            #div1
            {
                border-top: solid black;
                position: relative;
            }
            div div
            {
                background: blue;
                height: 1in;
                left: 1in;
                position: absolute;
                top: 1in;
            }
            img
            {
                bottom: 1in;
                height: auto;
                margin-bottom: 0.5in;
                margin-top: 0.5in;
                position: absolute;
                top: 0.5in;
            }
            div div, img
            {
                width: 1in;
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
  			1in   : 'height' (used width) / (intrinsic ratio)
		+
  			0     : 'padding-bottom'
		+
  			0     : 'border-bottom-width'
		+
  			0.5in : 'margin-bottom'
		+
  			1in   : 'bottom'
		===========
  			436px  : height of containing block (0px)

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
  			1in   : 'height' (used width) / (intrinsic ratio)
		+
  			0     : 'padding-bottom'
		+
  			0     : 'border-bottom-width'
		+
  			0.5in : 'margin-bottom'
		+
  			(solve): 'bottom'
		===================
 			240px  : height of containing block (0px)

  			So the solved bottom value should be -240px
	*/
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
