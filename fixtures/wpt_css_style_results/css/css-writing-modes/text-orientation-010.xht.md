# css/css-writing-modes/text-orientation-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-010.xht"
}
```

## style[0]

```css

				@font-face
				{
					font-family: "mplus-1p-regular";
					src: url("/fonts/mplus-1p-regular.woff") format("woff");
				}

				div
				{
					font-family: "mplus-1p-regular";
					font-size: 32px;
					line-height: 1.5; /* computes to 48px */
					margin-bottom: 10px;
					writing-mode: vertical-rl;
				}

				div#test
				{
					border: 1px solid gray;
				}

				span
				{
					text-orientation: mixed;					/* The property to be tested */
				}

		
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
