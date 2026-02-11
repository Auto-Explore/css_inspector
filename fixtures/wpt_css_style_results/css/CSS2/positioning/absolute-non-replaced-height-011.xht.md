# css/CSS2/positioning/absolute-non-replaced-height-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-011.xht"
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
                background: blue;
                bottom: 1in;
                height: auto;
                margin-bottom: auto;
                margin-top: auto;
                position: absolute;
                top: 1in;
                width: 100%;
            }

            /*

		 1in : top
	+
		 auto (to solve): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 auto (to solve) : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 auto (to solve): margin-bottom
	+
		1in : bottom
	=============
		3in  : height of containing block


	"
	'height' is 'auto', 'top' and 'bottom' are not 'auto',
	then 'auto' values for 'margin-top' and 'margin-bottom' are set to 0
	and solve for 'height'
	"

	so this brings:

		 1in : top
	+
		 0 (set): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 auto (to solve) : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 0 (set): margin-bottom
	+
		1in : bottom
	=============
		3in  : height of containing block

		So, height must use 1in in order to balance the equation

            */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
