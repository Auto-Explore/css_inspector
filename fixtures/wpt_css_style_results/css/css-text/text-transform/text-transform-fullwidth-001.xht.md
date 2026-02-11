# css/css-text/text-transform/text-transform-fullwidth-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-text/text-transform/text-transform-fullwidth-001.xht"
}
```

## style[0]

```css

			<![CDATA[
			        @font-face {
					font-family: 'mplus-1p-regular';
					src: url('/fonts/mplus-1p-regular.woff') format('woff');
			        }
				.test span {
					text-transform: full-width;
				}
				/* the CSS below is not part of the test */
				span {
				       font-family: 'mplus-1p-regular';
					background-color: cyan;
				}
				.test {
					color: Blue;
					line-height: 1.1;
				}
				.test div {
					display: inline-block;
					margin-bottom: .5em;
					text-align: center;
					white-space: nowrap;
				}
			]]>
		
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
