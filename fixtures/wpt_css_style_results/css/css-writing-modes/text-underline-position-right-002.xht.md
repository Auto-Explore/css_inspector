# css/css-writing-modes/text-underline-position-right-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-underline-position-right-002.xht"
}
```

## style[0]

```css
<![CDATA[

			div
			{
				writing-mode: 				vertical-rl;

			}

			span#ahem_text
			{
				position: absolute;
				left: 0;
				border: 1px dashed silver;

				font-family: ahem;
				font-size: 6.25em; /* equivalent to 100px */
				line-height: 1.0;
				color: orange;
			}

			span#sample_text_underline
			{
				position: absolute;
				left: 0;
				color:blue;

				text-decoration: underline;
				text-underline-position: right;
			}

			span#sample_text
			{
				color: silver;
				font-size: 6.25em; /* equivalent to 100px */
				line-height: 1.0;
			}


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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
