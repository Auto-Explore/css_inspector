# css/CSS2/positioning/absolute-replaced-width-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-009.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: ltr;
                height: 3in;
                position: relative;
                width: 3in;
            }
            svg
            {
                margin-left: auto;
                margin-right: auto;
                left: auto;
                position: absolute;
                right: auto;
            }

			/*

			In this test, the svg's containing block is div#div1.

			The contraining equation should be balanced in the following manner:

			left: will take the static position within its ltr containing block
			margin-left: will become 0
			width: will become 300px
			margin-right: will become 0
			right: will be the width of div#div1 minus 300px (288px minus 300px == -12px)

			*/

            div div
            {
                background: orange;
                height: 50px;
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
