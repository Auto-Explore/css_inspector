# css/CSS2/positioning/absolute-non-replaced-height-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-008.xht"
}
```

## style[0]

```css

            #div1
            {
                background: blue;
                height: 3in;
                position: relative;
                width: 1in;
            }
            div div
            {
                background: orange;
                bottom: auto;
                height: 1in;
                margin-bottom: auto;
                margin-top: auto;
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
		 1in : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 auto (to solve): margin-bottom
	+
		auto (to solve) : bottom
	=============
		3in  : height of containing block


	"
	'top' and 'bottom' are 'auto' and 'height' is not 'auto',
	then set 'top' to the static position,
	set 'auto' values for 'margin-top' and 'margin-bottom' to 0,
	and solve for 'bottom'
	"

	so this brings:

		 0 (solved static position) : top
	+
		 0 (solved): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 1in : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 0 (solved): margin-bottom
	+
		auto (to solve) : bottom
	=============
		3in  : height of containing block

		So, bottom must use 2in in order to balance the equation


            */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
