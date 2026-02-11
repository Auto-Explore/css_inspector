# css/CSS2/positioning/absolute-replaced-width-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-width-003.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                height: 3in;
                width: 3in;
            }
            svg
            {
                height: 100px;
                margin-left: auto;
                margin-right: auto;
                position: absolute;
            }

			/*

			In this test, the svg's containing block is the initial containing block.

			The contraining equation should be balanced in the following manner:

			left: will take the static position within its ltr containing block
			margin-left: will become 0
			width: will become 300px
			margin-right: will become 0
			right: will be the width of viewport minus 300px minus computed left value (body's margin-left plus border-left width == 11px)

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
