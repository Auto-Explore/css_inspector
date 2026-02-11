# css/CSS2/positioning/absolute-non-replaced-height-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-004.xht"
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
                bottom: 0.5in;
                height: 1in;
                margin-top: auto;
                margin-bottom: 0.5in;
                position: absolute;
                top: 0.5in;
                width: 100%;
            }

		/*

		0.5in : top
	+
		auto (solve): margin-top
	+
		0 : border-top-width
	+
		0 : padding-top
	+
		1in: height
	+
		0 : padding-bottom
	+
		0 : border-bottom-width
	+
		0.5in : margin-bottom
	+
		0.5in : bottom
	=============
		3.0in : height of containing block

	So, margin-top must use 0.5in so that the equation gets balanced.

		*/

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
