# css/CSS2/positioning/absolute-replaced-width-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-010.xht"
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
                height: 100px;
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
                height: 100px;
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
