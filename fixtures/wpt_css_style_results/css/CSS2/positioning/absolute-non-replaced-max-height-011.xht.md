# css/CSS2/positioning/absolute-non-replaced-max-height-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-max-height-011.xht"
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
                max-height: 0.5in;
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
		 auto (to solve) : height (may be constrained to use 0.5in by max-height)
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

		 1in : top
	+
		 0 (set): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 1in (not constrained) : height (must be constrained to use 0.5in by max-height)
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

		So, here, we must reenter the algorithm since height is
		constrained and is no longer 'auto'.

		"
		If none of the three [top, height, bottom] are 'auto':
		If both 'margin-top' and 'margin-bottom' are 'auto',
		solve the equation under the extra constraint that
		the two margins get equal values.
		"

	so this brings:

		 1in : top
	+
		 auto (to solve): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 0.5in (constrained) : height (constrained by max-height)
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


		So, here, margin-top must use 0.25in and margin-bottom must use 0.25in
		so that the equation remains balanced.

    */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
