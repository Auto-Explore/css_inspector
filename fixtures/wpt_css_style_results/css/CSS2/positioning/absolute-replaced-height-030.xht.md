# css/CSS2/positioning/absolute-replaced-height-030.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-030.xht"
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
                height: 15px;
                left: 15px;
                position: absolute;
                top: 1in;
                width: 15px;
            }
            img
            {
                bottom: 1in;
                height: auto;
                margin-bottom: 0.5in;
                margin-top: 0.5in;
                position: absolute;
                top: 0.5in;
                width: auto;
            }

			/*

			0.5in   : top
		+
			0.5in   : margin-top
		+
			15px    : intrinsic height
		+
			0.5in   : margin-bottom
		+
			1in     : bottom
		====================
		   255px != 0px (height of containing block)
		   Therefore, set bottom value must be ignored and
		   the equation must be solved for bottom.


			0.5in   : top
		+
			0.5in   : margin-top
		+
			15px    : intrinsic height
		+
			0.5in   : margin-bottom
		+
			(solve) : bottom
		====================
		   159px != 0px (height of containing block)
		   Therefore, solved bottom value must be -159px
			*/

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
