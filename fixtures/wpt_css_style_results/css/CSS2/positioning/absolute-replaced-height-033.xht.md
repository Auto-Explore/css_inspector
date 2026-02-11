# css/CSS2/positioning/absolute-replaced-height-033.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-033.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
                height: 2in;
            }
            div div
            {
                border: solid green;
                height: 1in;
                position: absolute;
                top: 1in;
                width: 300px;
            }
            iframe
            {
                border: solid red;
                bottom: 1in;
                margin-bottom: 0.5in;
                margin-top: 0.5in;
                position: absolute;
                top: 0.5in;
                width: auto;
            }

			/*

  			0.5in    : 'top'
		+
  			0.5in    : 'margin-top'
		+
  			3px      : 'border-top-width' medium which is often resolved as 3px
		+
  			0        : 'padding-top'
		+
  			1in      : 'height' 50% of height of containing block
		+
  			0        : 'padding-bottom'
		+
  			3px      : 'border-bottom-width' medium which is often resolved as 3px
		+
  			0.5in    : 'margin-bottom'
		+
  			1in      : 'bottom'
		================
  			342px    : height of containing block (192px)

		So, bottom has to be ignored and forced to have the value that
		will balance the equation. So, this brings up


  			0.5in   : 'top'
		+
  			0.5in   : 'margin-top'
		+
  			3px     : 'border-top-width' medium which is often resolved as 3px
		+
  			0       : 'padding-top'
		+
  			1in     : 'height' 50% of height of containing block
		+
  			0       : 'padding-bottom'
		+
  			3px     : 'border-bottom-width' medium which is often resolved as 3px
		+
  			0.5in   : 'margin-bottom'
		+
  			(solve) : 'bottom'
		================
  			-246px   : height of containing block (192px)

			So, the solved bottom value should be -54px .
			*/

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
