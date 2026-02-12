# css/CSS2/positioning/absolute-replaced-width-030.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-030.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: rtl;
                height: 296px;
                position: absolute;
                width: auto;
            }

			/*
			The svg's containing block (which is div#div1)
			has width: auto in which case shrink-to-fit width
			applies which is given by its non-positioned
			content, which is its inner div.
			*/

            svg
            {
                margin-left: auto;
                margin-right: auto;
                left: 1in;
                position: absolute;
                right: auto;
            }
            div div
            {
                background: orange;
                height: 50px;
                margin-left: 1in;
                margin-top: 50px;
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
