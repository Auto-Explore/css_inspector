# css/css-writing-modes/text-orientation-mixed-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-mixed-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
			@font-face
				{
					font-family: "mplus-1p-regular";
					src: url("/fonts/mplus-1p-regular.woff") format("woff");
					/* filesize: 803300 bytes (784.5 KBytes) */
				}

			div
				{
					float: left;
					color: blue;
					font-family: "mplus-1p-regular";
					font-size: 30px;
					line-height: 1.4; /* equivalent to 42px:
					so that top-half-leading outside content area is 6px
					and bottom-half-leading outside content area is 6px */
					margin: 10px;
				}

			div#vertical
				{
					border: 1px solid gray;
					text-orientation:	mixed;
					writing-mode:	vertical-rl;
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
