# css/CSS2/positioning/absolute-replaced-width-031.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-031.xht"
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
                width: auto;
            }

			/*
			The svg's containing block is div#div1 which
			has width: auto. Therefore, shrink-to-fit width
			will apply to div#div1.
			Its non-positioned content is its
			inner div which requires a
			minimum of 288px. Therefore,
			div#div1 shrink-to-fit width
			will compute to 288px.
			*/

            svg
            {
                height: 100px;
                margin-left: auto;
                margin-right: auto;
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
			388px		: width of containing block (div#div1 width is 288px)

			Therefore, used right offset must be -100px so that the
			constraining equation gets balanced.

			*/


            div div
            {
                background: orange;
                height: 100px;
                margin-left: 88px;
                margin-top: 100px;
                width: 200px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
