# css/CSS2/positioning/absolute-non-replaced-max-height-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-max-height-010.xht"
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
                height: 2in;
                margin-bottom: auto;
                margin-top: auto;
                max-height: 1in;
                position: absolute;
                top: auto;
                width: 100%;
            }

            /*

		 auto (to solve) : top
	+
		 auto (to solve): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 2in : height (will be constrained to use 1in by max-height)
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
	'top' is 'auto', 'height' and 'bottom' are not 'auto',
	then set 'auto' values for 'margin-top' and 'margin-bottom' to 0,
	and solve for 'top'
	"

	so this brings:

		 auto (to solve) : top
	+
		 0 (set): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 1in (constrained) : height
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

		So, top must use 1in in order to balance the equation

    */
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
