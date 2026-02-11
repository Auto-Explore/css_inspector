# css/css-writing-modes/text-orientation-sideways-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-sideways-001.xht"
}
```

## style[0]

```css
<![CDATA[
			@font-face
			{
				font-family: "DejaVuSerifBook";
				src: url("support/DejaVuSerif-webfont.woff") format("woff");
				/* Filesize: 18096 bytes (17.7 KBytes) */
			}

			div {
				border: 1px solid gray;
				float: left;
				color: blue;
				font-family: "DejaVuSerifBook";
				font-size: 30px;
				line-height: 1.4; /* equivalent to 42px:
				so that top-half-leading outside content is 6px
				and bottom-half-leading outside content is 6px */
				margin: 10px;
			}

			div#vertical
			{
				writing-mode: 				vertical-rl;
				text-orientation:			sideways;
			}

		    img {vertical-align: top;}

		]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “src”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
