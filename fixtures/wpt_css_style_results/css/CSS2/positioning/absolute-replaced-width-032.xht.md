# css/CSS2/positioning/absolute-replaced-width-032.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-032.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: rtl;
                height: 3in;
                position: absolute;
            }
            #div2
            {
                height: 110px;
                position: relative;
            }
            #div3
            {
                background: orange;
                height: 1in;
                margin-left: 88px;
                width: 200px;
            }
            svg
            {
                left: 88px;
                position: absolute;
                right: auto;
            }

			/*

			88px		: left
		+
			0px (set)	: margin-left
		+
			300px		: width (pre-defined fallback when intrinsic values are not defined)
		+
			0px (set)	: margin-right
		+
			(solve)		: right
		=========================
			388px		: width of containing block (div#div2 width is 288px)

			Therefore, used right offset must be -100px so that the
			constraining equation gets balanced.

			*/

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
