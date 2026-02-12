# css/CSS2/positioning/absolute-non-replaced-height-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-007.xht"
}
```

## style[0]

```css

            #div1
            {
                color: orange;
                font: 100px/1 Ahem;
                height: 300px;
                position: relative;
                width: 200px;
            }
            div div
            {
                background: blue;
                bottom: 200px;
                height: auto;
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
		 auto (to solve) : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 auto (to solve): margin-bottom
	+
		200px : bottom
	=============
		300px  : height of containing block

		"
		'top' and 'height' are 'auto' and 'bottom' is not 'auto',
		then the height is based on the content per 10.6.7,
		set 'auto' values for 'margin-top' and 'margin-bottom' to 0,
		and solve for 'top'
		"

		so this brings:


		 auto (to solve) : top
	+
		 0 (solved): margin-top
	+
		 0 : border-top-width
	+
		 0 : padding-top
	+
		 100px (solved: based on content) : height
	+
		 0 : padding-bottom
	+
		 0 : border-bottom-width
	+
		 0 (solved) : margin-bottom
	+
		200px : bottom
	=============
		300px  : height of containing block

		so top will use 0px

		*/

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
