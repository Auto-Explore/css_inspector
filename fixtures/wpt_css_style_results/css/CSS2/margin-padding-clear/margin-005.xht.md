# css/CSS2/margin-padding-clear/margin-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-005.xht"
}
```

## style[0]

```css

            #wrapper
            {
                position: relative;
            }
            #div1, #div2
            {
                border: 10px solid green;
            }
            #div1, #reference
            {
                left: 0;
                position: absolute;
                top: 0;
            }

			/*
			In this test, #div1, which is absolutely positioned, is
			#div2's containing block. In which case, the width of #div1 is
			shrink-to-fit to the width of #div2 plus its borders.
			http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
			*/

            #div2
            {
                height: 1in;
                margin: auto;
                width: 3in;
            }
            #reference
            {
                border: 20px solid red;
                height: 96px;
                width: 288px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
