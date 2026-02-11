# css/CSS2/positioning/absolute-non-replaced-height-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-height-006.xht"
}
```

## style[0]

```css

            #div1
            {
                border: 10px solid black;
                height: 300px;
                position: relative;
                width: 300px;
                overflow: auto;
            }
            div div
            {
                background: blue;
                height: 150px;
                margin-bottom: 50px;
                margin-top: 50px;
                position: absolute;
                top: 50px;
                width: 50%;
            }
            #div2
            {
                bottom: 50px;
            }

		/*

		50px : top
	+
		50px : margin-top
	+
		0 : border-top-width
	+
		0 : padding-top
	+
		150px : height
	+
		0 : padding-bottom
	+
		0 : border-bottom-width
	+
		50px : margin-bottom
	+
		50px : bottom
	=============
		350px  while the height of containing block is only 300px.

		So, here we definitely have an overconstrained situation. In which
		case the spec clearly states
		"If the values are over-constrained, ignore the value for 'bottom'
		and solve for that value."
		http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-height

		So, here, the used value for bottom will be 0px so that the equation
		gets balanced.

		*/

            #div3
            {
                bottom: 0;
                left: 50%;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
