# css/CSS2/positioning/absolute-replaced-width-029.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-029.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: rtl;
                height: 2in;
                position: relative;
                width: 2in;
            }
            img
            {
                margin-left: auto;
                margin-right: auto;
                left: 1in;
                position: absolute;
                right: auto;
            }

			/*

			1in			: left
			0px (set)	: margin-left
			1in			: width (intrinsic width)
			0px (set)	: margin-right
			(solve)		: right
		=========================
		    2in			: width of containing block (2in)

		    Therefore , the used right offset must be 0px
		    so that the contraining equation gets balanced.

			*/

            div div
            {
                background: orange;
                height: 96px;
                margin-left: 1in;
                margin-top: 96px;
                width: 96px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
