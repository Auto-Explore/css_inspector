# css/css-writing-modes/text-orientation-upright-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-upright-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
			@font-face
			{
					font-family: "mplus-1p-regular";
					src: url("/fonts/mplus-1p-regular.woff") format("woff");
			}

			div {
				float: left;
				color: blue;
				font-family: "mplus-1p-regular";
				font-size: 24px; /* we must try to avoid a too tall div which could generate 2 line boxes; so we use a smaller font size */
				line-height: 1.5; /* equivalent to 36px:
				so that top-half-leading outside content is 6px
				and bottom-half-leading outside content is 6px */
				margin: 10px;
			}

			div#vertical
			{
				border: 1px solid gray;
				writing-mode: 				vertical-rl;
				text-orientation:   		upright;
			}

		    img {vertical-align: top;}

		]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
